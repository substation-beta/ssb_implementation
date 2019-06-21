// Imports
use ssb_parser::data::SsbRender;
use image::RgbaImage;
use super::error::RenderingError;


/// Consumes ssb data and renders them on images
pub struct SsbRenderer {
    data: SsbRender
}
impl SsbRenderer {
    pub fn new(data: SsbRender) -> Self {
        Self {
            data
        }
    }
    pub fn render(&self, img: &mut RgbaImage) -> Result<&Self,RenderingError> {

        // TODO: whole rendering process
        if self.data.events.is_empty() { image::imageops::colorops::invert(img); }

        Ok(self)
    }
}