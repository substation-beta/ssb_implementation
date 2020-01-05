pub trait FloatExt<T> {
    fn clamp(self, min: T, max: T) -> T;    // Stabilization: <https://doc.rust-lang.org/std/primitive.f32.html#method.clamp>
    fn round_half_down(self) -> T;
}
impl FloatExt<f32> for f32 {
    #[inline]
    fn clamp(self, min: f32, max: f32) -> f32 {
        self.max(min).min(max)
    }
    #[inline]
    fn round_half_down(self) -> f32 {
        if self.fract() <= 0.5 {self.floor()} else {self.ceil()}
    }
}

pub trait RangeExt {
    fn is_empty(&self) -> bool; // Stabilization: <https://doc.rust-lang.org/std/ops/struct.Range.html#method.is_empty>
}
impl RangeExt for std::ops::Range<u16> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.end <= self.start
    }
}