// Imports
use ssb_parser::data::SsbRender;
use image::RgbaImage;
use super::error::RenderingError;


/// Renderer for ssb data on images
pub struct SsbRenderer {
    data: SsbRender
}
impl SsbRenderer {
    /// Consumes ssb data as rendering blueprint
    pub fn new(data: SsbRender) -> Self {
        Self {
            data
        }
    }
    /// Renders on image by ssb and returns new image
    pub fn render(&self, img: RgbaImage) -> Result<RgbaImage,RenderingError> {
        let (width, height) = img.dimensions();
        let mut buffer = img.into_raw();


        // TODO: whole rendering process
        if self.data.target_depth > 0 {
            for channel in &mut buffer {
                *channel = 255u8 - *channel;
            }
        }


        Ok(RgbaImage::from_raw(width, height, buffer).expect("Couldn't repack image buffer?!"))
    }
}