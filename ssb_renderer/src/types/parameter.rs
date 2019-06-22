/// Condition to trigger rendering on specific image.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderTrigger<'a> {
    Id(&'a str),
    Time(u32)
}

/// Re-export RGBA image from image crate for rendering in- & output.
pub use image::RgbaImage;