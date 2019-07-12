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
    image::{ColorType,ImageView},
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
        let color_type = ColorType::RGB24;
        let stride = color_type.row_size(width);
        let mut data = vec![0u8;height as usize * stride as usize];
        renderer.render(
            ImageView::new(width, height, stride, color_type, vec![&mut data]).expect("ImageView must've valid dimensions!"),
            RenderTrigger::Id("test")
        ).expect("Image rendering mustn't fail!");


    });
}