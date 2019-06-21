// Imports
use std::{
    convert::TryFrom,
    time::Duration
};
use ssb_parser::data::{Ssb,SsbRender};
use ssb_renderer::rendering::SsbRenderer;
use microbench::{bench, Options};
use image::RgbaImage;


// Benchmark
fn main() {
    // Test data
    let renderer = SsbRenderer::new(SsbRender::try_from(Ssb::default()).expect("Default ssb cannot be wrong for rendering!"));
    // Run test
    bench(&Options::default().time(Duration::from_secs(2)), "Basic rendering.", || {


        // TODO: more complex rendering
        let img = RgbaImage::new(1920, 1080);
        let _new_img = renderer.render(img).expect("Image rendering mustn't fail!");


    });
}