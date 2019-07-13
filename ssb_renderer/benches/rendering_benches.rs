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
use image::RgbImage;
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
        let img = RgbImage::new(1920, 1080);
        let (width, height, stride, color_type, mut data) = (img.width(), img.height(), img.sample_layout().height_stride, ColorType::RGB24, img.into_raw());
        renderer.render(
            ImageView::new(width as u16, height as u16, stride as u32, color_type, vec![&mut data]).expect("ImageView must've valid dimensions!"),
            RenderTrigger::Id("test")
        ).expect("Image rendering mustn't fail!");
        let _img = RgbImage::from_raw(width, height, data).expect("Image rebuild mustn't fail!");
        

    });
}