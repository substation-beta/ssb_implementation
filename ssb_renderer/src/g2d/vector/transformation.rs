// Imports
use std::ops::Mul;
use super::{
    types::Coordinate,
    point::{Point,ORIGIN_POINT}
};
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;


// Aligned memory for fast SSE result storage
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(align(128))]
#[derive(Default)]
struct M128 {
    data: [f32; 4]
}

// Matrix without any transformation
const IDENTITY_MATRIX: [f32; 16] = [
    1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 1., 0.,
    0., 0., 0., 1.
];

// 4x4 matrix for transformation of points
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    repr(align(128))
)]
pub struct Transformation {
    pub matrix: [f32; 16]
}
impl Default for Transformation {
    fn default() -> Self {
        Self {
            matrix: IDENTITY_MATRIX
        }
    }
}
impl Mul for Transformation {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        // Output buffer
        let mut transformation = Self {
            matrix: [0_f32; 16]
        };
        // Calculation native
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        for row_index in (0..16).step_by(4) {
            for column in 0..4 {
                for offset in 0..4 {
                    transformation.matrix[row_index + column] += self.matrix[row_index + offset] * other.matrix[column + (offset << 2)];
                }
            }
        }
        // Calculation with SSE
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            // Preload
            let other_row_x = _mm_load_ps(other.matrix.as_ptr());
            let other_row_y = _mm_load_ps(other.matrix.as_ptr().add(4));
            let other_row_z = _mm_load_ps(other.matrix.as_ptr().add(8));
            let other_row_w = _mm_load_ps(other.matrix.as_ptr().add(12));
            // Calculate rows
            for row_index in (0..16).step_by(4) {
                _mm_store_ps(
                    transformation.matrix.as_mut_ptr().add(row_index),
                    _mm_add_ps(
                        _mm_mul_ps(_mm_set1_ps(self.matrix[row_index]), other_row_x),
                        _mm_add_ps(
                            _mm_mul_ps(_mm_set1_ps(self.matrix[row_index + 1]), other_row_y),
                            _mm_add_ps(
                                _mm_mul_ps(_mm_set1_ps(self.matrix[row_index + 2]), other_row_z),
                                _mm_mul_ps(_mm_set1_ps(self.matrix[row_index + 3]), other_row_w)
                            )
                        )
                    )
                );
            }
        }
        // Return filled buffer
        transformation
    }
}
impl Transformation {
    pub fn is_identity(&self) -> bool {
        self.matrix == IDENTITY_MATRIX
    }
    pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
        self * Transformation {
            matrix: [
                1., 0., 0., x,
                0., 1., 0., y,
                0., 0., 1., z,
                0., 0., 0., 1.
            ]
        }
    }
    pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
        self * Transformation {
            matrix: [
                x, 0., 0., 0.,
                0., y, 0., 0.,
                0., 0., z, 0.,
                0., 0., 0., 1.
            ]
        }
    }
    pub fn shear(self, x: f32, y: f32) -> Self {
        self * Transformation {
            matrix: [
                1., x, 0., 0.,
                y, 1., 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.
            ]
        }
    }
    pub fn rotate_x(self, rad: f32) -> Self {
        let (sin, cos) = (rad.sin(), rad.cos());
        self * Transformation {
            matrix: [
                1., 0., 0., 0.,
                0., cos, -sin, 0.,
                0., sin, cos, 0.,
                0., 0., 0., 1.
            ]
        }
    }
    pub fn rotate_y(self, rad: f32) -> Self {
        let (sin, cos) = (rad.sin(), rad.cos());
        self * Transformation {
            matrix: [
                cos, 0., sin, 0.,
                0., 1., 0., 0.,
                -sin, 0., cos, 0.,
                0., 0., 0., 1.
            ]
        }
    }
    pub fn rotate_z(self, rad: f32) -> Self {
        let (sin, cos) = (rad.sin(), rad.cos());
        self * Transformation {
            matrix: [
                cos, -sin, 0., 0.,
                sin, cos, 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.
            ]
        }
    }
    pub fn transform<'origin, I: IntoIterator<Item = &'origin mut Point>>(&self, point_iter: I, z: Coordinate) {
        // Calculation native
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            let col2z_col3 = [
                self.matrix[2] * z + self.matrix[3],
                self.matrix[6] * z + self.matrix[7],
                self.matrix[14] * z + self.matrix[15]
            ];
            for point in point_iter {
                *point = {
                    let (x, y, w) = (
                        self.matrix[0] * point.x + self.matrix[1] * point.y + col2z_col3[0],
                        self.matrix[4] * point.x + self.matrix[5] * point.y + col2z_col3[1],
                        self.matrix[12] * point.x + self.matrix[13] * point.y + col2z_col3[2]
                    );
                    point4d_to_2d(x, y, w)
                }
            }
        }
        // Calculation with SSE
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            let mut result = M128::default();
            let (col0, col1, col2z_col3) = (
                _mm_set_ps(self.matrix[12], self.matrix[8], self.matrix[4], self.matrix[0]),
                _mm_set_ps(self.matrix[13], self.matrix[9], self.matrix[5], self.matrix[1]),
                _mm_add_ps(
                    _mm_mul_ps(_mm_set_ps(self.matrix[14], self.matrix[10], self.matrix[6], self.matrix[2]), _mm_set1_ps(z)),
                    _mm_set_ps(self.matrix[15], self.matrix[11], self.matrix[7], self.matrix[3])
                )
            );
            for point in point_iter {
                *point = {
                    _mm_store_ps(
                        result.data.as_mut_ptr(),
                        _mm_add_ps(
                            _mm_mul_ps(col0, _mm_set1_ps(point.x)),
                            _mm_add_ps(
                                _mm_mul_ps(col1, _mm_set1_ps(point.y)),
                                col2z_col3
                            )
                        )
                    );
                    point4d_to_2d(result.data[0], result.data[1], result.data[3])
                }
            }
        }
    }
}

// Helpers
#[inline]
fn point4d_to_2d(x: f32, y: f32, w: f32) -> Point {
    if w == 1.0 {
        Point {x, y}
    } else if w == 0.0 {
        ORIGIN_POINT
    } else {
        Point {
            x: x / w,
            y: y / w
        }
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::{Transformation,Point};

    #[test]
    fn matrix_multiplication() {
        assert_eq!(
            Transformation { matrix: [
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 5.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
            ]} *
            Transformation {matrix: [
                    1.0, 0.0, 0.0, 9.0,
                    0.0, 1.0, 0.0, 8.0,
                    0.0, 0.0, 1.0, 7.0,
                    0.0, 0.0, 0.0, 1.0
            ]},
            Transformation {
                matrix: [
                    1.0, 0.0, 0.0, 9.0,
                    0.0, 5.0, 0.0, 40.0,
                    0.0, 0.0, 1.0, 7.0,
                    0.0, 0.0, 0.0, 1.0
                ]
            }
        );
    }

    #[test]
    fn matrix_translate_scale() {
        assert_eq!(
            Transformation::default()
            .scale(1., 5., 1.)
            .translate(9., 8., 7.),
            Transformation {
                matrix: [
                    1.0, 0.0, 0.0, 9.0,
                    0.0, 5.0, 0.0, 40.0,
                    0.0, 0.0, 1.0, 7.0,
                    0.0, 0.0, 0.0, 1.0
                ]
            }
        );
    }

    #[test]
    fn matrix_shear_rotate() {
        // Shear
        assert_eq!(
            Transformation::default()
            .shear(-1., 2.),
            Transformation {
                matrix: [
                    1.0, -1.0, 0.0, 0.0,
                    2.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
                ]
            }
        );
        // Rotate
        assert_eq!(
            Transformation::default()
            .rotate_z(90_f32.to_radians()),
            Transformation {
                matrix: [
                    -0.00000004371139, -1.0, 0.0, 0.0,
                    1.0, -0.00000004371139, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
                ]
            }
        );
    }

    #[test]
    fn matrix_transform() {
        // Orthogonal
        let mut points = [
            Point {x: -5., y: 0.},
            Point {x: 5., y: 0.},
            Point {x: 0., y: 2.24}
        ];
        Transformation::default()
        .scale(1., 5., 1.)
        .translate(9., 8., 7.)
        .transform(points.iter_mut(), 0.);
        assert_eq!(
            points,
            [
                Point {x: 4., y: 40.},
                Point {x: 14., y: 40.},
                Point {x: 9., y: 51.2}
            ]
        );
        // Perspective
        /*
        let mut points = [
            Point {x: 0., y: 0.},
            Point {x: 0., y: 2.}
        ];
        Transformation::default()
        .rotate_y(90_f32.to_radians())
        .transform_perspective(points.iter_mut(), 1., 50);
        assert_eq!(
            points,
            [
                Point {x: 1., y: 0.},
                Point {x: 1., y: 2.}
            ]
        );
        */
    }
}