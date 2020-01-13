// Imports
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
compile_error!("x86 or x86_64 target architecture required!");

use std::ops::Mul;


// Aligned memory for fast SSE results store
#[repr(align(128))]
#[derive(Default)]
struct M128 {
    data: [f32; 4]
}

// Matrix without any transformation
const IDENTITY_MATRIX: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
];

// 4x4 matrix for transformation of points
#[derive(Debug, PartialEq, Clone)]
#[repr(align(128))]
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
        // Storages
        let mut matrix = [0_f32; 16];
        let mut temp = M128::default();
        // Calculation
        for dimension in 0..4 {
            for column in 0..4 {
                unsafe {
                    _mm_store_ps(
                        temp.data.as_mut_ptr(),
                        _mm_mul_ps(
                            _mm_load_ps(self.matrix.as_ptr().add(dimension << 2)),
                            _mm_set_ps(other.matrix[12+column], other.matrix[8+column], other.matrix[4+column], other.matrix[0+column])
                        )
                    );
                    matrix[(dimension << 2) + column] = temp.data[0] + temp.data[1] + temp.data[2] + temp.data[3];
                }
            }
        }
        Self {
            matrix
        }
    }
}
impl Transformation {
    pub fn is_identity(&self) -> bool {
        self.matrix == IDENTITY_MATRIX
    }

    pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
        // Storages
        let mut matrix = self.matrix.clone();
        let mut temp = M128::default();
        // Calculation
        unsafe {
            let translation = _mm_set_ps(1.0, z, y, x);
            for dimension in 0..4 {
                _mm_store_ps(
                    temp.data.as_mut_ptr(),
                    _mm_mul_ps(
                        _mm_load_ps(self.matrix.as_ptr().add(dimension << 2)),
                        translation
                    )
                );
                matrix[(dimension << 2) + 3] = temp.data[0] + temp.data[1] + temp.data[2] + temp.data[3];
            }
        }
        Self {
            matrix
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