// Imports
use ssb_parser::{
    SsbRender,
    objects::ssb_objects::EventTrigger
};
use super::{
    g2d::image::ImageView,
    types::{
        error::RenderingError,
        parameter::RenderTrigger
    }
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
    pub fn render<'data>(&mut self, mut img: ImageView<'data>, trigger: RenderTrigger) -> Result<ImageView<'data>,RenderingError> {
        // Find match of render and ssb trigger
        for event in &self.data.events {
            if match (&event.trigger, trigger) {
                (EventTrigger::Id(event_id), RenderTrigger::Id(render_id)) => event_id == render_id,
                (EventTrigger::Time((start_ms, end_ms)), RenderTrigger::Time(current_ms)) => (start_ms..end_ms).contains(&&current_ms),
                _ => false
            } {


                // TODO: whole rendering process
                for row in img.plane_rows_mut(0).expect("One plane should always exist!") {
                    for sample in row {
                        *sample = std::u8::MAX - *sample;
                    }
                }


            }
        }
        // Return still valid image reference
        Ok(img)
    }
}