// Imports
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use glutin::{WindowBuilder, ContextBuilder, GlRequest, Api, GlProfile, EventsLoop, ContextTrait};
use glutin::dpi::LogicalSize;
use log::info;
use super::error::GlError;
use super::safe::{Framebuffer, Renderbuffer, Texture2D, Viewport};

// GL environment by hidden window in separate thread
pub struct GlEnvironment<DataTypeIn, DataTypeOut> {
    worker_thread: Option<thread::JoinHandle<()>>,
    gl_sender: SyncSender<Option<DataTypeIn>>,
    user_receiver: Receiver<DataTypeOut>
}
impl<DataTypeIn, DataTypeOut> GlEnvironment<DataTypeIn, DataTypeOut> {
    // Constructor
    pub fn new<WorkerType>(version: (u8, u8), worker: WorkerType) -> Self
        where WorkerType: (Fn(DataTypeIn) -> DataTypeOut) + std::marker::Send + 'static,
            DataTypeIn: std::marker::Send + 'static,
            DataTypeOut: std::marker::Send + 'static {
        // Channels between user & worker
        let (gl_sender, gl_receiver) = sync_channel::<Option<DataTypeIn>>(0);
        let (user_sender, user_receiver) = sync_channel::<DataTypeOut>(0);
        // Return instance
        Self{
            // Work in new thread for separate context
            worker_thread: Some(thread::spawn(move ||{
                info!("Started GlEnvironment thread.");
                // Create OpenGL context
                let gl_window = ContextBuilder::new()
                    .with_gl(GlRequest::Specific(Api::OpenGl, version))
                    .with_gl_profile(GlProfile::Core)
                    .build_windowed(
                        WindowBuilder::new()
                            .with_dimensions(LogicalSize::new(1.0, 1.0))
                            .with_visibility(false),
                        &EventsLoop::new()
                    ).expect(&format!("Unsupported GL version: {:?}!", &version));
                unsafe {
                    gl_window.make_current().expect("GL context binding not possible!");
                }
                gl32::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
                // Process data with worker
                loop {
                    if let Ok(may_data) = gl_receiver.recv() {
                        // User data to process
                        if let Some(data) = may_data {
                            user_sender.send(worker(data)).ok();
                        // Stop signal from Drop
                        } else {
                            break;
                        }
                    }
                }
                info!("Finished GlEnvironment thread.");
            })),
            gl_sender,
            user_receiver
        }
    }
    // Methods
    pub fn process(&self, data: DataTypeIn) -> Result<DataTypeOut, GlError>
        where DataTypeIn: std::marker::Send + 'static,
            DataTypeOut: std::marker::Send + 'static {
        // Send data to worker
        self.gl_sender.send(Some(data))?;
        // Receive worker result
        Ok(self.user_receiver.recv()?)
    }
}
impl<DataTypeIn, DataTypeOut> Drop for GlEnvironment<DataTypeIn, DataTypeOut> {
    // Deconstructor
    fn drop(&mut self) {
        // Send stop signal into thread
        self.gl_sender.send(None).ok();
        // Wait for thread to finish
        self.worker_thread
            .take().expect("Thread join handle should have been reserved for drop!")
            .join().expect("Thread join failed, unexpected termination!");
    }
}


// Supported color types
pub enum ColorType {
    RGB,
    BGR,
    RGBA,
    BGRA
}
impl ColorType {
    pub fn size(&self) -> u8 {
        match self {
            ColorType::RGB | ColorType::BGR => 3,
            ColorType::RGBA | ColorType::BGRA => 4
        }
    }
    pub fn gl_enum(&self) -> gl32::types::GLenum {
        match self {
            ColorType::RGB => gl32::RGB,
            ColorType::BGR => gl32::BGR,
            ColorType::RGBA => gl32::RGBA,
            ColorType::BGRA => gl32::BGRA
        }
    }
}


// GL offscreen resources available in context
pub struct OffscreenContext {
    // Size
    width: u16,
    height: u16,
    color_type: ColorType,
    samples: u8,
    // Transfer
    fb_tex: Framebuffer,
    tex_color: Texture2D,
    // Draw
    fb_render: Framebuffer,
    _rb_color: Renderbuffer,
    _rb_depth_stencil: Renderbuffer
}
impl OffscreenContext {
    // Constructor
    pub fn new(width: u16, height: u16, color_type: ColorType, samples: u8) -> Result<Self,GlError> {
        // Create transfer texture
        let tex_color = Texture2D::generate();
        tex_color.bind();
        Texture2D::tex_image_2d(gl32::RGBA, width, height, color_type.gl_enum(), gl32::UNSIGNED_BYTE, None);
        Texture2D::unbind();
        // Create framebuffer for transfer texture
        let fb_tex = Framebuffer::generate();
        fb_tex.bind();
        Framebuffer::texture_2d(gl32::COLOR_ATTACHMENT0, &tex_color);
        if Framebuffer::status() != gl32::FRAMEBUFFER_COMPLETE {
            Framebuffer::unbind();
            return Err(GlError::new("Couldn't create texture framebuffer!"));
        }
        Framebuffer::unbind();

        // Create multisampled renderbuffer for color
        let rb_color = Renderbuffer::generate();
        rb_color.bind();
        Renderbuffer::storage_multisample(samples, gl32::RGBA8, width, height);
        // Create multisampled renderbuffer for depth & stencil
        let rb_depth_stencil = Renderbuffer::generate();
        rb_depth_stencil.bind();
        Renderbuffer::storage_multisample(samples, gl32::DEPTH24_STENCIL8, width, height);
        Renderbuffer::unbind();
        // Create framebuffer for rendering
        let fb_render = Framebuffer::generate();
        fb_render.bind();
        Framebuffer::renderbuffer(gl32::COLOR_ATTACHMENT0, &rb_color);
        Framebuffer::renderbuffer(gl32::DEPTH_STENCIL_ATTACHMENT, &rb_depth_stencil);
        if Framebuffer::status() != gl32::FRAMEBUFFER_COMPLETE {
            Framebuffer::unbind();
            return Err(GlError::new("Couldn't create rendering framebuffer!"));
        }
        Framebuffer::unbind();

        // Return resources
        Ok(Self {
            width,
            height,
            color_type,
            samples,
            fb_tex,
            tex_color,
            fb_render,
            _rb_color: rb_color,
            _rb_depth_stencil: rb_depth_stencil
        })
    }

    // Getters
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn color_type(&self) -> &ColorType {
        &self.color_type
    }
    pub fn samples(&self) -> u8 {
        self.samples
    }

    // Methods
    pub fn process<CB>(&self, buffer: &mut [u8], callback: CB) -> Result<(), GlError>
        where CB: FnOnce() {
        // Check buffer size enough for context
        if buffer.len() < self.width as usize * self.height as usize * self.color_type.size() as usize {
            return Err(GlError::new("Buffer size too small!"));
        }
        // Upload image into texture
        self.tex_color.bind();
        Texture2D::tex_sub_image_2d(0, 0, self.width, self.height, self.color_type.gl_enum(), gl32::UNSIGNED_BYTE, buffer);
        Texture2D::unbind();
        // Copy texture into renderbuffer
        self.fb_tex.bind_target(gl32::READ_FRAMEBUFFER);
        self.fb_render.bind_target(gl32::DRAW_FRAMEBUFFER);
        Framebuffer::blit(0, 0, self.width as i32, self.height as i32, 0, 0, self.width as i32, self.height as i32, gl32::COLOR_BUFFER_BIT, gl32::NEAREST);
        Framebuffer::unbind();
        // Setup renderbuffer viewport
        Viewport(0, 0, self.width, self.height);
        // Draw on renderbuffer
        self.fb_render.bind();
        callback();
        Framebuffer::unbind();
        // Copy renderbuffer into texture
        self.fb_render.bind_target(gl32::READ_FRAMEBUFFER);
        self.fb_tex.bind_target(gl32::DRAW_FRAMEBUFFER);
        Framebuffer::blit(0, 0, self.width as i32, self.height as i32, 0, 0, self.width as i32, self.height as i32, gl32::COLOR_BUFFER_BIT, gl32::NEAREST);
        Framebuffer::unbind();
        // Download image from texture
        self.tex_color.bind();
        Texture2D::get_tex_image(self.color_type.gl_enum(), gl32::UNSIGNED_BYTE, buffer);
        Texture2D::unbind();
        // All worked
        Ok(())
    }
}