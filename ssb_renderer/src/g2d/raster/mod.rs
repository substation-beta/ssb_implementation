/// Image components for rastering.
pub mod image;
/// Alpha image with offset position.
pub mod mask;
/// Rasterize to convert path to mask.
pub mod rasterize;

// Scanline detection for flat paths.
mod scanlines;