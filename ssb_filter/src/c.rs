// Imports
use libc::*;
use ssb_renderer::{
    ssb_parser::data::{Ssb, SsbRender},
    rendering::SsbRenderer,
    image::{ColorType, ImageView},
    types::parameter::RenderTrigger
};
use std::{
    convert::TryFrom,
    error::Error,
    ffi::CStr,
    fs::File,
    io::{BufRead, BufReader, Cursor},
    ptr::null_mut,
    slice::{from_raw_parts, from_raw_parts_mut}
};


// Helpers
fn error_to_c(error: Box<Error>, error_message: *mut c_char, error_message_capacity: c_ushort) {
    if !error_message.is_null() && error_message_capacity > 0 {
        let mut msg = error.to_string();
        msg.truncate((error_message_capacity-1) as usize);
        msg.push('\0');
        unsafe {msg.as_ptr().copy_to(error_message as *mut u8, msg.len());}
    }
}
fn into_ptr<T>(data: T) -> *mut c_void {
    Box::into_raw(Box::new(data)) as *mut c_void
}
fn free_ptr(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {Box::from_raw(ptr);}
    }
}

/// Get library version as C string.
#[no_mangle]
pub extern fn ssb_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Create renderer instance by file.
/// 
/// **file** mustn't be *null*.
/// 
/// **error_message** can be *null*.
/// 
/// Returns renderer instance or *null*.
#[no_mangle]
pub extern fn ssb_new_renderer_by_file(file: *const c_char, error_message: *mut c_char, error_message_capacity: c_ushort) -> *mut c_void {
    match ssb_new_renderer_by_file_inner(file) {
        Ok(renderer) => into_ptr(renderer),
        Err(error) => {
            error_to_c(error, error_message, error_message_capacity);
            null_mut()
        }
    }
}
fn ssb_new_renderer_by_file_inner(file: *const c_char) -> Result<SsbRenderer, Box::<Error>> {
    ssb_new_renderer_inner(BufReader::new(
        File::open(unsafe{ CStr::from_ptr(file) }.to_str()?)?
    ))
}
fn ssb_new_renderer_inner<R: BufRead>(script: R) -> Result<SsbRenderer, Box::<Error>> {
    Ok(SsbRenderer::new(
        Ssb::default().parse_owned(script)
        .and_then(|ssb| SsbRender::try_from(ssb) )?
    ))
}

/// Create renderer instance by script content.
/// 
/// **script** mustn't be *null*.
/// 
/// **error_message** can be *null*.
/// 
/// Returns renderer instance or *null*.
#[no_mangle]
pub extern fn ssb_new_renderer_by_script(script: *const c_char, error_message: *mut c_char, error_message_capacity: c_ushort) -> *mut c_void {
    match ssb_new_renderer_by_script_inner(script) {
        Ok(renderer) => into_ptr(renderer),
        Err(error) => {
            error_to_c(error, error_message, error_message_capacity);
            null_mut()
        }
    }
}
fn ssb_new_renderer_by_script_inner(script: *const c_char) -> Result<SsbRenderer, Box::<Error>> {
    ssb_new_renderer_inner(
        Cursor::new(unsafe{ CStr::from_ptr(script) }.to_str()?)
    )
}

/// Destroy renderer instance.
/// 
/// **renderer** can be *null*.
#[no_mangle]
pub extern fn ssb_destroy_renderer(renderer: *mut c_void) {
    free_ptr(renderer);
}

/// Render on image by time.
/// 
/// **renderer** can be *null*.
/// 
/// **color_type** mustn't be *null*.
/// 
/// **planes** mustn't be *null* and contains enough pointers with enough data for given **color_type**.
/// 
/// **error_message** can be *null*.
/// 
/// Returns 0 on success, 1 on error.
#[no_mangle]
pub extern fn ssb_render_by_time(
    renderer: *mut c_void,
    width: c_ushort, height: c_ushort, stride: c_uint, color_type: *const c_char, planes: *const *mut c_uchar,
    time: c_uint,
    error_message: *mut c_char, error_message_capacity: c_ushort
) -> c_int {
    match ssb_render_by_time_inner(renderer, width, height, stride, color_type, planes, time) {
        Ok(()) => 0,
        Err(error) => {
            error_to_c(error, error_message, error_message_capacity);
            1
        }
    }
}
fn ssb_render_by_time_inner(
    renderer: *mut c_void,
    width: c_ushort, height: c_ushort, stride: c_uint, color_type: *const c_char, planes: *const *mut c_uchar,
    time: c_uint
) -> Result<(), Box<Error>> {
    ssb_render_inner(
        renderer,
        width, height, stride, color_type, planes,
        RenderTrigger::Time(time)
    )
}
fn ssb_render_inner(
    renderer: *mut c_void,
    width: c_ushort, height: c_ushort, stride: c_uint, color_type: *const c_char, planes: *const *mut c_uchar,
    trigger: RenderTrigger
) -> Result<(), Box<Error>> {
    if !renderer.is_null() {
        unsafe {
            let color_type = ColorType::by_name( CStr::from_ptr(color_type).to_str()? )?;
            (*(renderer as *mut SsbRenderer)).render(
                ImageView::new(
                    width,
                    height,
                    stride,
                    color_type,
                    {
                        let min_data_size = height as usize * stride as usize;
                        from_raw_parts(planes, color_type.planes() as usize)
                        .iter()
                        .map(|plane| from_raw_parts_mut(*plane, min_data_size) )
                        .collect()
                    }
                )?,
                trigger
            )?;
        }
    }
    Ok(())
}

/// Render on image by id.
/// 
/// **renderer** can be *null*.
/// 
/// **color_type** mustn't be *null*.
/// 
/// **planes** mustn't be *null* and contains enough pointers with enough data for given **color_type**.
/// 
/// **id** mustn't be *null*.
/// 
/// **error_message** can be *null*.
/// 
/// Returns 0 on success, 1 on error.
#[no_mangle]
pub extern fn ssb_render_by_id(
    renderer: *mut c_void,
    width: c_ushort, height: c_ushort, stride: c_uint, color_type: *const c_char, planes: *const *mut c_uchar,
    id: *const c_char,
    error_message: *mut c_char, error_message_capacity: c_ushort
) -> c_int {
    match ssb_render_by_id_inner(renderer, width, height, stride, color_type, planes, id) {
        Ok(()) => 0,
        Err(error) => {
            error_to_c(error, error_message, error_message_capacity);
            1
        }
    }
}
fn ssb_render_by_id_inner(
    renderer: *mut c_void,
    width: c_ushort, height: c_ushort, stride: c_uint, color_type: *const c_char, planes: *const *mut c_uchar,
    id: *const c_char
) -> Result<(), Box<Error>> {
    ssb_render_inner(
        renderer,
        width, height, stride, color_type, planes,
        RenderTrigger::Id(unsafe{ CStr::from_ptr(id) }.to_str()?)
    )
}