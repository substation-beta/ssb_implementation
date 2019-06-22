// Imports
use ssb_parser::{
    data::SsbRender,
    types::ssb::EventTrigger
};
use super::types::{
    error::RenderingError,
    parameter::{RgbaImage,RenderTrigger}
};


/// Renderer for ssb data on images.
#[derive(Debug, PartialEq, Clone)]
pub struct SsbRenderer {
    data: SsbRender
}
impl SsbRenderer {
    /// Consumes ssb data as rendering blueprint.
    pub fn new(data: SsbRender) -> Self {
        Self {
            data
        }
    }
    /// Renders on image by ssb matching trigger.
    pub fn render(&mut self, img: RgbaImage, trigger: RenderTrigger) -> Result<RgbaImage,RenderingError> {
        // Unpack image
        let (width, height) = img.dimensions();
        let mut buffer = img.into_raw();
        // Find match of render and ssb trigger
        for event in &self.data.events {
            if match (&event.trigger, trigger) {
                (EventTrigger::Id(event_id), RenderTrigger::Id(render_id)) => event_id == render_id,
                (EventTrigger::Time((start_ms, end_ms)), RenderTrigger::Time(current_ms)) => (start_ms..end_ms).contains(&&current_ms),
                _ => false
            } {


                // TODO: whole rendering process
                for channel in &mut buffer {
                    *channel = std::u8::MAX - *channel;
                }


            }
        }
        // Pack image for client
        Ok(RgbaImage::from_raw(width, height, buffer).expect("Couldn't repack image buffer?!"))
    }
}