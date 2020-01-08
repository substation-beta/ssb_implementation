// See <https://github.com/YaLTeR/vapoursynth-rs/tree/master/vapoursynth/src/plugins>

// Imports
use vapoursynth::{
    prelude::*,
    plugins::*,
    core::CoreRef,
    video_info::VideoInfo,
    make_filter_function,
    export_vapoursynth_plugin
};
use failure::{Error, err_msg, format_err, bail};
use ssb_renderer::{
    ssb_parser::{Ssb,SsbRender},
    image::{ColorType,ImageView},
    RenderTrigger,
    SsbRenderer
};
use std::{
    io::{BufRead,BufReader,Cursor},
    fs::File,
    convert::TryFrom,
    sync::Mutex,
    cell::RefCell,
    slice::from_raw_parts_mut
};

// Register functions to plugin
export_vapoursynth_plugin! {
    // Plugin configuration
    Metadata {
        // Internal unique key
        identifier: "de.youka.ssb",
        // Namespace/prefix of plugin functions
        namespace: "ssb",
        // Plugin description
        name: "SSB subtitle plugin.",
        // Plugin does changes? (optimization)
        read_only: false
    },
    // Plugin functions
    [
        RenderFunction::new(),
        RenderRawFunction::new()
    ]
}

// Create vapoursynth functions
make_filter_function! {
    // Name rust & vapoursynth function
    RenderFunction, "render"
    // Vapoursynth function call
    fn create_render<'core>(
        _api: API,
        _core: CoreRef<'core>,
        clip: Node<'core>,
        script: &[u8]
    ) -> Result<Option<Box<dyn Filter<'core> + 'core>>, Error> {
        Ok(Some(Box::new(
            build_render_filter(clip, BufReader::new(
                File::open(
                    String::from_utf8( script.to_vec() )?
                )?
            ))?
        )))
    }
}
make_filter_function! {
    // Name rust & vapoursynth function
    RenderRawFunction, "render_raw"
    // Vapoursynth function call
    fn create_render_raw<'core>(
        _api: API,
        _core: CoreRef<'core>,
        clip: Node<'core>,
        data: &[u8]
    ) -> Result<Option<Box<dyn Filter<'core> + 'core>>, Error> {
        Ok(Some(Box::new(
            build_render_filter(clip, Cursor::new(data))?
        )))
    }
}

// Build vapoursynth filter instance
fn build_render_filter<R>(clip: Node, reader: R) -> Result<RenderFilter, Error>
    where R: BufRead {
    Ok(RenderFilter{
        source: clip,
        renderer: Mutex::new(RefCell::new(SsbRenderer::new(
            Ssb::default().parse_owned(reader)
            .and_then(SsbRender::try_from)
            .map_err(|err| err_msg(err.to_string()) )?
        )))
    })
}

// Filter class
struct RenderFilter<'core> {
    source: Node<'core>,
    renderer: Mutex<RefCell<SsbRenderer>>
}
impl<'core> Filter<'core> for RenderFilter<'core> {
    // Output video meta information
    fn video_info(&self, _api: API, _core: CoreRef<'core>) -> Vec<VideoInfo<'core>> {
        // Just take from local video node
        vec![self.source.info()]
    }

    // Fetch frame from pipeline for local filter
    fn get_frame_initial(
        &self,
        _api: API,
        _core: CoreRef<'core>,
        context: FrameContext,
        n: usize
    ) -> Result<Option<FrameRef<'core>>, Error> {
        // Just fetch it, nothing more
        self.source.request_frame_filter(context, n);
        Ok(None)
    }

    // Process available frame
    fn get_frame(
        &self,
        _api: API,
        core: CoreRef<'core>,
        context: FrameContext,
        n: usize
    ) -> Result<FrameRef<'core>, Error> {
        // Get frame
        let frame = self.source
            .get_frame_filter(context, n)
            .ok_or_else(|| format_err!("Couldn't get the source frame!"))?;
        // Check RGB(A) format
        let format = frame.format();
        if format.color_family() == ColorFamily::RGB && (3..4).contains(&format.plane_count()) && format.sample_type() == SampleType::Integer && format.bits_per_sample() == 8 {
            // Create lock on renderer
            if let Ok(renderer_refcell) = self.renderer.lock() {
                // Make frame copy
                let mut frame = FrameRefMut::copy_of(core, &frame);
                // Edit frame by SSB
                renderer_refcell.borrow_mut().render(
                    ImageView::new(
                        frame.width(0) as u16,
                        frame.height(0) as u16,
                        frame.stride(0) as u32,
                        if format.plane_count() == 4 {ColorType::R8G8B8A8} else {ColorType::R8G8B8},
                        unsafe {
                            // Serve color planes
                            let frame_size = frame.height(0) * frame.stride(0);
                            if format.plane_count() == 4 {
                                vec![
                                    from_raw_parts_mut(frame.data_ptr_mut(0), frame_size),
                                    from_raw_parts_mut(frame.data_ptr_mut(1), frame_size),
                                    from_raw_parts_mut(frame.data_ptr_mut(2), frame_size),
                                    from_raw_parts_mut(frame.data_ptr_mut(3), frame_size)
                                ]
                            } else {
                                vec![
                                    from_raw_parts_mut(frame.data_ptr_mut(0), frame_size),
                                    from_raw_parts_mut(frame.data_ptr_mut(1), frame_size),
                                    from_raw_parts_mut(frame.data_ptr_mut(2), frame_size)
                                ]
                            }
                        }
                    ).map_err(|err| err_msg(err.to_string()) )?,
                    RenderTrigger::Time(
                        // Calculate frame time (in milliseconds)
                        match self.source.info().framerate {
                            Property::Constant(framerate) => (framerate.denominator as f64 / framerate.numerator as f64 * 1000.0 * n as f64) as u32,
                            Property::Variable => { // Reserved frame properties: <http://www.vapoursynth.com/doc/apireference.html#reserved-frame-properties>
                                let frame_props = frame.props();
                                if let (Ok(duration_numerator), Ok(duration_denominator)) = (frame_props.get_int("_DurationNum"), frame_props.get_int("_DurationDen")) {
                                    (duration_numerator as f64 / duration_denominator as f64 * 1000.0) as u32
                                } else {
                                    bail!("Couldn't get frame time! No constant framerate or variable frame property.")
                                }
                            }
                        }
                    )
                ).map_err(|err| err_msg(err.to_string()) )?;
                // Pass processed frame copy further through the pipeline
                Ok(frame.into())
            } else {
                bail!("Couldn't lock renderer!")
            }
        } else {
            bail!("Frame format must be RGB24 or RGBA32!")
        }
    }
}