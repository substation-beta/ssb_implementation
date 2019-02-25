// Imports
use microbench::{bench, Options};
use std::time::Duration;
use ssb_renderer::gl_utils::environment::{GlEnvironment, OffscreenContext, ColorType};

// Benchmark
fn main() {
    GlEnvironment::new((3, 2), |()| {
        let offscreen_context = OffscreenContext::new(1920, 1080, ColorType::RGBA, 8).unwrap();
        let mut buffer = vec![0u8; offscreen_context.width() as usize * offscreen_context.height() as usize * offscreen_context.color_type().size() as usize];
        bench(&Options::default().time(Duration::from_secs(2)), "Offscreen context processing overhead", || {
            offscreen_context.process(&mut buffer, || {}).unwrap();
        });
    }).process(()).unwrap();
}