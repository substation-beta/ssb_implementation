// See <https://github.com/YaLTeR/vapoursynth-rs/tree/master/vapoursynth/src/plugins>

// Imports
use vapoursynth::prelude::*;
use vapoursynth::plugins::*;
use vapoursynth::core::CoreRef;
use vapoursynth::video_info::{VideoInfo};
use vapoursynth::{make_filter_function, export_vapoursynth_plugin};
use failure::{Error, format_err, bail};

// Filter structure
struct Render<'core> {
    source: Node<'core>,
    _script: String
}
impl<'core> Filter<'core> for Render<'core> {
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
            .ok_or_else(|| format_err!("Couldn't get the source frame"))?;

        // Validate frame format
        if frame.format().sample_type() == SampleType::Float {
            bail!("Floating point formats are not supported");
        }

        // Make frame copy
        let mut frame = FrameRefMut::copy_of(core, &frame);

        // Iterate through color planes of frame
        for plane in 0..frame.format().plane_count() {
            // Iterate through pixel rows
            for row in 0..frame.height(plane) {
                // Get sample sizes
                let bits_per_sample = frame.format().bits_per_sample();
                let bytes_per_sample = frame.format().bytes_per_sample();
                // Invert row pixels value by color depth
                match bytes_per_sample {
                    // 8-bit
                    1 => for pixel in frame.plane_row_mut::<u8>(plane, row) {
                        *pixel = 255 - *pixel;
                    },
                    // 16-bit
                    2 => for pixel in frame.plane_row_mut::<u16>(plane, row) {
                        *pixel = ((1u64 << bits_per_sample) - 1) as u16 - *pixel;
                    },
                    // 32-bit (that's a lot)
                    4 => for pixel in frame.plane_row_mut::<u32>(plane, row) {
                        *pixel = ((1u64 << bits_per_sample) - 1) as u32 - *pixel;
                    },
                    _ => unreachable!(),
                }
            }
        }

        // Pass processed frame copy further the pipeline
        Ok(frame.into())
    }
}

// Create vapoursynth function of filter type
make_filter_function! {
    // Name rust & vapoursynth function
    RenderFunction, "render"
    // Vapoursynth function call
    fn create_render<'core>(
        _api: API,
        _core: CoreRef<'core>,
        clip: Node<'core>,
        script: &[u8]
    ) -> Result<Option<Box<Filter<'core> + 'core>>, Error> {
        // Convert script name safely to UTF-8
        let script_utf8 = String::from_utf8(script.to_vec())?;
        // Return custom filter instance
        Ok(Some(Box::new(
            Render{source: clip, _script: script_utf8}
        )))
    }
}

// Register filters to plugin
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
        read_only: false,
    },
    // Plugin functions
    [
        RenderFunction::new()
    ]
}