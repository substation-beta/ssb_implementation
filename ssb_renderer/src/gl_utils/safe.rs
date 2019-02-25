// Imports
use std::ffi::{CStr, CString};
use std::os::raw::{c_char};
use std::ptr::{null, null_mut};
use std::mem::size_of;
use super::error::GlError;

// Helper macro
macro_rules! check_loaded {
    ($name:ident, $body:expr) => {
        if gl32::$name::is_loaded() {
            $body
        } else {
            panic!("{} not loaded!", stringify!($name));
        }
    }
}


// FUNCTIONS
// GetString
#[allow(non_snake_case)]
pub fn GetString(name: gl32::types::GLenum) -> Option<String> {
    check_loaded!(
        GetString,
        unsafe {
            let gl_string = gl32::GetString(name);
            if gl_string == null() {
                None
            } else {
                Some(CStr::from_ptr(gl_string as *const c_char).to_string_lossy().to_string())
            }
        }
    )
}

// GetError
#[allow(non_snake_case)]
pub fn GetError() -> gl32::types::GLenum {
    check_loaded!(
        GetError,
        unsafe {
            gl32::GetError()
        }
    )
}

// ClearColor
#[allow(non_snake_case)]
pub fn ClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
    check_loaded!(
        ClearColor,
        unsafe {
            gl32::ClearColor(red, green, blue, alpha);
        }
    );
}

// Clear
#[allow(non_snake_case)]
pub fn Clear(mask: gl32::types::GLenum) {
    check_loaded!(
        Clear,
        unsafe {
            gl32::Clear(mask);
        }
    );
}

// Viewport
#[allow(non_snake_case)]
pub fn Viewport(x: u16, y: u16, width: u16, height: u16) {
    check_loaded!(
        Viewport,
        unsafe {
            gl32::Viewport(
                x as gl32::types::GLint, y as gl32::types::GLint,
                width as gl32::types::GLsizei, height as gl32::types::GLsizei
            );
        }
    );
}

// Enable / Disable
#[allow(non_snake_case)]
pub fn Enable(cap: gl32::types::GLenum) {
    check_loaded!(
        Enable,
        unsafe {
            gl32::Enable(cap);
        }
    );
}
#[allow(non_snake_case)]
pub fn Disable(cap: gl32::types::GLenum) {
    check_loaded!(
        Disable,
        unsafe {
            gl32::Disable(cap);
        }
    );
}

// Blending
#[allow(non_snake_case)]
pub fn BlendFunc(sfactor: gl32::types::GLenum, dfactor: gl32::types::GLenum) {
    check_loaded!(
        BlendFunc,
        unsafe {
            gl32::BlendFunc(sfactor, dfactor);
        }
    );
}
#[allow(non_snake_case)]
pub fn BlendFuncSeparate(srcRGB: gl32::types::GLenum, dstRGB: gl32::types::GLenum, srcAlpha: gl32::types::GLenum, dstAlpha: gl32::types::GLenum) {
    check_loaded!(
        BlendFuncSeparate,
        unsafe {
            gl32::BlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
        }
    );
}
#[allow(non_snake_case)]
pub fn BlendEquation(mode: gl32::types::GLenum) {
    check_loaded!(
        BlendEquation,
        unsafe {
            gl32::BlendEquation(mode);
        }
    );
}


// OBJECTS
// Framebuffer
pub struct Framebuffer {
    id: gl32::types::GLuint
}
impl Framebuffer {
    // New
    pub fn generate() -> Self {
        check_loaded!(
            GenFramebuffers,
            {
                let mut id: gl32::types::GLuint = 0;
                unsafe {
                    gl32::GenFramebuffers(1, &mut id);
                }
                Self{
                    id
                }
            }
        )
    }
    // Bind
    pub fn bind(&self) {
        check_loaded!(
            BindFramebuffer,
            unsafe {
                gl32::BindFramebuffer(gl32::FRAMEBUFFER, self.id);
            }
        );
    }
    pub fn bind_target(&self, target: gl32::types::GLenum) {
        check_loaded!(
            BindFramebuffer,
            unsafe {
                gl32::BindFramebuffer(target, self.id);
            }
        );
    }
    pub fn unbind() {
        check_loaded!(
            BindFramebuffer,
            unsafe {
                gl32::BindFramebuffer(gl32::FRAMEBUFFER, 0);
            }
        );
    }
    // Check complete status
    pub fn status() -> gl32::types::GLenum {
        check_loaded!(
            CheckFramebufferStatus,
            unsafe {
                gl32::CheckFramebufferStatus(gl32::FRAMEBUFFER)
            }
        )
    }
    // Link
    pub fn texture_2d(attachment: gl32::types::GLenum, texture: &Texture2D) {
        check_loaded!(
            FramebufferTexture2D,
            unsafe {
                gl32::FramebufferTexture2D(gl32::FRAMEBUFFER, attachment, gl32::TEXTURE_2D, texture.id, 0);
            }
        );
    }
    pub fn renderbuffer(attachment: gl32::types::GLenum, renderbuffer: &Renderbuffer) {
        check_loaded!(
            FramebufferRenderbuffer,
            unsafe {
                gl32::FramebufferRenderbuffer(gl32::FRAMEBUFFER, attachment, gl32::RENDERBUFFER, renderbuffer.id);
            }
        );
    }
    // Blit
    pub fn blit(src_x0: i32, src_y0: i32, src_x1: i32, src_y1: i32,
        dst_x0: i32, dst_y0: i32, dst_x1: i32, dst_y1: i32,
        mask: gl32::types::GLbitfield, filter: gl32::types::GLenum) {
        check_loaded!(
            BlitFramebuffer,
            unsafe {
                gl32::BlitFramebuffer(
                    src_x0, src_y0, src_x1, src_y1,
                    dst_x0, dst_y0, dst_x1, dst_y1,
                    mask, filter
                );
            }
        );
    }
}
impl Drop for Framebuffer {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteFramebuffers,
            unsafe {
                gl32::DeleteFramebuffers(1, &self.id);
            }
        );
    }
}

// Texture (2D)
pub struct Texture2D {
    id: gl32::types::GLuint
}
impl Texture2D {
    // New
    pub fn generate() -> Self {
        check_loaded!(
            GenTextures,
            {
                let mut id: gl32::types::GLuint = 0;
                unsafe {
                    gl32::GenTextures(1, &mut id);
                }
                Self{
                    id
                }
            }
        )
    }
    // Bind
    pub fn bind(&self) {
        check_loaded!(
            BindTexture,
            unsafe {
                gl32::BindTexture(gl32::TEXTURE_2D, self.id);
            }
        );
    }
    pub fn unbind() {
        check_loaded!(
            BindTexture,
            unsafe {
                gl32::BindTexture(gl32::TEXTURE_2D, 0);
            }
        );
    }
    // Memory
    pub fn tex_image_2d(internalformat: gl32::types::GLenum, width: u16, height: u16, data_format: gl32::types::GLenum, data_type: gl32::types::GLenum, data: Option<&[u8]>) {
        check_loaded!(
            TexImage2D,
            unsafe {
                gl32::TexImage2D(
                    gl32::TEXTURE_2D, 0, internalformat as gl32::types::GLint,
                    width as gl32::types::GLsizei, height as gl32::types::GLsizei, 0,
                    data_format, data_type, data.map_or(null(), |bytes| bytes.as_ptr() as *const _)
                );
            }
        );
    }
    pub fn tex_sub_image_2d(xoffset: i16, yoffset: i16, width: u16, height: u16, data_format: gl32::types::GLenum, data_type: gl32::types::GLenum, data: &[u8]) {
        check_loaded!(
            TexSubImage2D,
            unsafe {
                gl32::TexSubImage2D(
                    gl32::TEXTURE_2D, 0,
                    xoffset as gl32::types::GLint, yoffset as gl32::types::GLint, width as gl32::types::GLsizei, height as gl32::types::GLsizei,
                    data_format, data_type, data.as_ptr() as *const _
                );
            }
        );
    }
    pub fn get_tex_image(data_format: gl32::types::GLenum, data_type: gl32::types::GLenum, data: &mut [u8]) {
        check_loaded!(
            GetTexImage,
            unsafe {
                gl32::GetTexImage(gl32::TEXTURE_2D, 0, data_format, data_type, data.as_ptr() as *mut _);
            }
        );
    }
}
impl Drop for Texture2D {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteTextures,
            unsafe {
                gl32::DeleteTextures(1, &self.id);
            }
        );
    }
}

// Renderbuffer (with multisampling)
pub struct Renderbuffer {
    id: gl32::types::GLuint
}
impl Renderbuffer {
    // New
    pub fn generate() -> Self {
        check_loaded!(
            GenRenderbuffers,
            {
                let mut id: gl32::types::GLuint = 0;
                unsafe {
                    gl32::GenRenderbuffers(1, &mut id);
                }
                Self{
                    id
                }
            }
        )
    }
    // Bind
    pub fn bind(&self) {
        check_loaded!(
            BindRenderbuffer,
            unsafe {
                gl32::BindRenderbuffer(gl32::RENDERBUFFER, self.id);
            }
        );
    }
    pub fn unbind() {
        check_loaded!(
            BindRenderbuffer,
            unsafe {
                gl32::BindRenderbuffer(gl32::RENDERBUFFER, 0);
            }
        );
    }
    // Memory
    pub fn storage_multisample(samples: u8, internalformat: gl32::types::GLenum, width: u16, height: u16) {
        check_loaded!(
            RenderbufferStorageMultisample,
            unsafe {
                gl32::RenderbufferStorageMultisample(
                    gl32::RENDERBUFFER, samples as gl32::types::GLsizei, internalformat,
                    width as gl32::types::GLsizei, height as gl32::types::GLsizei
                );
            }
        );
    }
}
impl Drop for Renderbuffer {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteRenderbuffers,
            unsafe {
                gl32::DeleteRenderbuffers(1, &self.id);
            }
        );
    }
}

// VBO (Vertex buffer object)
pub struct VBO {
    id: gl32::types::GLuint
}
impl VBO {
    // New
    pub fn generate() -> Self {
        check_loaded!(
            GenBuffers,
            {
                let mut id: gl32::types::GLuint = 0;
                unsafe {
                    gl32::GenBuffers(1, &mut id);
                }
                Self{
                    id
                }
            }
        )
    }
    // Bind
    pub fn bind(&self) {
        check_loaded!(
            BindBuffer,
            unsafe {
                gl32::BindBuffer(gl32::ARRAY_BUFFER, self.id);
            }
        );
    }
    pub fn unbind() {
        check_loaded!(
            BindBuffer,
            unsafe {
                gl32::BindBuffer(gl32::ARRAY_BUFFER, 0);
            }
        );
    }
    // Memory
    pub fn data(data: &[f32]) {
        check_loaded!(
            BufferData,
            unsafe {
                gl32::BufferData(gl32::ARRAY_BUFFER, (data.len() * size_of::<f32>()) as gl32::types::GLsizeiptr, data.as_ptr() as *const _, gl32::STATIC_DRAW);
            }
        );
    }
    // Attributes
    pub fn enable_attrib_array(index: u32) {
        check_loaded!(
            EnableVertexAttribArray,
            unsafe {
                gl32::EnableVertexAttribArray(index);
            }
        );
    }
    pub fn disable_attrib_array(index: u32) {
        check_loaded!(
            DisableVertexAttribArray,
            unsafe {
                gl32::DisableVertexAttribArray(index);
            }
        );
    }
    pub fn attrib_pointer(index: u32, size: i32, stride: i32, offset: isize) {
        check_loaded!(
            VertexAttribPointer,
            unsafe {
                gl32::VertexAttribPointer(
                    index, size, gl32::FLOAT, gl32::FALSE,
                    stride * size_of::<f32>() as i32, (offset * size_of::<f32>() as isize) as *const _
                );
            }
        );
    }
    // Draw
    pub fn draw_arrays(mode: gl32::types::GLenum, first: u16, count: u16) {
        check_loaded!(
            DrawArrays,
            unsafe {
                gl32::DrawArrays(mode, first as gl32::types::GLint, count as gl32::types::GLsizei);
            }
        );
    }
}
impl Drop for VBO {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteBuffers,
            unsafe {
                gl32::DeleteBuffers(1, &self.id);
            }
        );
    }
}

// EBO (Element buffer object)
pub struct EBO {
    id: gl32::types::GLuint
}
impl EBO {
    // New
    pub fn generate() -> Self {
        check_loaded!(
            GenBuffers,
            {
                let mut id: gl32::types::GLuint = 0;
                unsafe {
                    gl32::GenBuffers(1, &mut id);
                }
                Self{
                    id
                }
            }
        )
    }
    // Bind
    pub fn bind(&self) {
        check_loaded!(
            BindBuffer,
            unsafe {
                gl32::BindBuffer(gl32::ELEMENT_ARRAY_BUFFER, self.id);
            }
        );
    }
    pub fn unbind() {
        check_loaded!(
            BindBuffer,
            unsafe {
                gl32::BindBuffer(gl32::ELEMENT_ARRAY_BUFFER, 0);
            }
        );
    }
    // Memory
    pub fn data(data: &[u32]) {
        check_loaded!(
            BufferData,
            unsafe {
                gl32::BufferData(gl32::ELEMENT_ARRAY_BUFFER, (data.len() * size_of::<u32>()) as gl32::types::GLsizeiptr, data.as_ptr() as *const _, gl32::STATIC_DRAW);
            }
        );
    }
    // Draw
    pub fn draw_elements(mode: gl32::types::GLenum, count: u16) {
        check_loaded!(
            DrawElements,
            unsafe {
                gl32::DrawElements(mode, count as gl32::types::GLsizei, gl32::UNSIGNED_INT, null() as *const _);
            }
        );
    }
}
impl Drop for EBO {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteBuffers,
            unsafe {
                gl32::DeleteBuffers(1, &self.id);
            }
        );
    }
}

// VAO (Vertex array object)
pub struct VAO {
    id: gl32::types::GLuint
}
impl VAO {
    // New
    pub fn generate() -> Self {
        check_loaded!(
            GenVertexArrays,
            {
                let mut id: gl32::types::GLuint = 0;
                unsafe {
                    gl32::GenVertexArrays(1, &mut id);
                }
                Self{
                    id
                }
            }
        )
    }
    // Bind
    pub fn bind(&self) {
        check_loaded!(
            BindVertexArray,
            unsafe {
                gl32::BindVertexArray(self.id);
            }
        );
    }
    pub fn unbind() {
        check_loaded!(
            BindVertexArray,
            unsafe {
                gl32::BindVertexArray(0);
            }
        );
    }
}
impl Drop for VAO {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteVertexArrays,
            unsafe {
                gl32::DeleteVertexArrays(1, &self.id);
            }
        );
    }
}

// Shader
pub struct Shader {
    id: gl32::types::GLuint
}
impl Shader {
    // New
    pub fn create(shader_type: gl32::types::GLenum) -> Self {
        check_loaded!(
            CreateShader,
            unsafe {
                Self {
                    id: gl32::CreateShader(shader_type)
                }
            }
        )
    }
    // Source
    pub fn source(&self, string: &str) {
        check_loaded!(
            ShaderSource,
            unsafe {
                let source = CString::new(string).expect("Source string shouldn't contain null bytes!");
                gl32::ShaderSource(
                    self.id, 1,
                    &source.as_ptr() as *const *const gl32::types::GLchar,
                    null()
                );
            }
        );
    }
    pub fn compile(&self) -> Result<(), GlError> {
        check_loaded!(
            CompileShader,
            unsafe {
                gl32::CompileShader(self.id);
                let mut success: gl32::types::GLint = 0;
                gl32::GetShaderiv(self.id, gl32::COMPILE_STATUS, &mut success);
                if success == 0 {
                    const BUF_SIZE: gl32::types::GLsizei = 1024;
                    let mut info_log: [gl32::types::GLchar; BUF_SIZE as usize] = [0; BUF_SIZE as usize];
                    gl32::GetShaderInfoLog(self.id, BUF_SIZE, null_mut(), info_log.as_mut_ptr());
                    return Err(GlError::new(
                        &CStr::from_ptr(info_log.as_ptr()).to_string_lossy().to_string()
                    ));
                }
                Ok(())
            }
        )
    }
}
impl Drop for Shader {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteShader,
            unsafe {
                gl32::DeleteShader(self.id);
            }
        );
    }
}

// Program
pub struct Program {
    id: gl32::types::GLuint
}
impl Program {
    // New
    pub fn create() -> Self {
        check_loaded!(
            CreateProgram,
            unsafe {
                Self {
                    id: gl32::CreateProgram()
                }
            }
        )
    }
    // Attach
    pub fn attach(&self, shader: &Shader) {
        check_loaded!(
            AttachShader,
            unsafe {
                gl32::AttachShader(self.id, shader.id);
            }
        );
    }
    // Link
    pub fn link(&self) -> Result<(), GlError> {
        check_loaded!(
            LinkProgram,
            unsafe {
                gl32::LinkProgram(self.id);
                let mut success: gl32::types::GLint = 0;
                gl32::GetProgramiv(self.id, gl32::LINK_STATUS, &mut success);
                if success == 0 {
                    const BUF_SIZE: gl32::types::GLsizei = 1024;
                    let mut info_log: [gl32::types::GLchar; BUF_SIZE as usize] = [0; BUF_SIZE as usize];
                    gl32::GetProgramInfoLog(self.id, BUF_SIZE, null_mut(), info_log.as_mut_ptr());
                    return Err(GlError::new(
                        &CStr::from_ptr(info_log.as_ptr()).to_string_lossy().to_string()
                    ));
                }
                Ok(())
            }
        )
    }
    // Use
    pub fn using(&self) {
        check_loaded!(
            UseProgram,
            unsafe {
                gl32::UseProgram(self.id);
            }
        );
    }
    // Attributes
    pub fn attrib_location(&self, name: &str) -> gl32::types::GLint {
        check_loaded!(
            GetAttribLocation,
            unsafe {
                gl32::GetAttribLocation(self.id, CString::new(name).expect("Name string shouldn't contain null bytes!").as_ptr())
            }
        )
    }
}
impl Drop for Program {
    // Delete
    fn drop(&mut self) {
        check_loaded!(
            DeleteProgram,
            unsafe {
                gl32::DeleteProgram(self.id);
            }
        );
    }
}