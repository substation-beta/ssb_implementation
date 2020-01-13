// Imports
use std::ops::Mul;


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
        // Calculation with SSE
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            // Extra imports
            #[cfg(target_arch = "x86")]
            use std::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
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
        // Calculation native
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        for row_index in (0..16).step_by(4) {
            for column in 0..4 {
                transformation.matrix[row_index + column] =
                    self.matrix[row_index] * other.matrix[column] +
                    self.matrix[row_index + 1] * other.matrix[column + 4] +
                    self.matrix[row_index + 2] * other.matrix[column + 8] +
                    self.matrix[row_index + 3] * other.matrix[column + 12];
            }
        }
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

    // TODO: scale
    // TODO: rotate
    // TODO: shear
    // TODO: transform_orthogonal
    // TODO: transform_perspective

}


// Tests
#[cfg(test)]
mod tests {
    use super::Transformation;

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
    fn matrix_translate() {
        assert_eq!(
            Transformation { matrix: [
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 5.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
            ]}.translate(9.0, 8.0, 7.0),
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
}