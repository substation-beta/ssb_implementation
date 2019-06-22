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
    rendering::SsbRenderer,
    types::parameter::{RgbaImage,RenderTrigger},
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
    bench(&Options::default().time(Duration::from_secs(2)), "Basic rendering.", || {


        // TODO: more complex rendering
        let img = RgbaImage::new(1920, 1080);
        let _new_img = renderer.render(img, &RenderTrigger::Id("test")).expect("Image rendering mustn't fail!");


    });
}