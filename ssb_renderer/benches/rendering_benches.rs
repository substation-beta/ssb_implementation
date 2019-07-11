// Imports
use std::{
    convert::TryFrom,
    time::Duration
};
use ssb_parser::{
    data::{Ssb,SsbRender},
    types::ssb::{Event,EventTrigger}
};
use ssb_renderer::{
    image::ImageView,
    types::parameter::RenderTrigger,
    rendering::SsbRenderer
};
use microbench::{bench, Options};


// Benchmark
fn main() {
    // Test data
    let mut ssb = Ssb::default();
    ssb.events.push(Event {
        trigger: EventTrigger::Id("test".to_owned()),
        macro_name: None,
        note: None,
        data: "".to_owned(),
        data_location: (0,0)
    });
    let mut renderer = SsbRenderer::new(SsbRender::try_from(ssb).expect("Ssb was certainly valid!"));
    // Run test
    bench(&Options::default().time(Duration::from_secs(3)), "Basic rendering.", || {


        // TODO: more complex rendering
        let width = 1920u16;
        let height = 1080u16;
        let sample_size = 3u8;  // RGB24
        let stride = width as u32 * sample_size as u32;
        let mut rgb24_data = vec![0u8;height as usize * stride as usize];
        renderer.render(
            ImageView::new_rgb24(width, height, stride, &mut rgb24_data).expect("ImageView must've valid dimensions!"),
            RenderTrigger::Id("test")
        ).expect("Image rendering mustn't fail!");


    });
}