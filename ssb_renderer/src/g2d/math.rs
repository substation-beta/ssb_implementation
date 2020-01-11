pub trait FloatExt<T> {
    fn clamp(self, min: T, max: T) -> T;    // Stabilization: <https://doc.rust-lang.org/std/primitive.f32.html#method.clamp>
    fn round_half_down(self) -> T;
    fn eq_close(self, other: T) -> bool;
}
macro_rules! impl_FloatExt {
    ($T:tt) => {
        impl FloatExt<$T> for $T {
            #[inline]
            fn clamp(self, min: $T, max: $T) -> $T {
                self.max(min).min(max)
            }
            #[inline]
            fn round_half_down(self) -> $T {
                if self.fract() <= 0.5 {self.floor()} else {self.ceil()}
            }
            #[inline]
            fn eq_close(self, other: $T) -> bool {
                (self - other).abs() < std::$T::EPSILON
            }
        }
    }
}
impl_FloatExt!(f32);
impl_FloatExt!(f64);

pub trait RangeExt {
    fn is_empty(&self) -> bool; // Stabilization: <https://doc.rust-lang.org/std/ops/struct.Range.html#method.is_empty>
}
impl RangeExt for std::ops::Range<u16> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.end <= self.start
    }
}