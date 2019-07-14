// Imports
use libc::*;
use ssb_renderer::{
    ssb_parser::data::{Ssb, SsbRender},
    rendering::SsbRenderer
};
use std::{
    convert::TryFrom,
    error::Error,
    ffi::CStr,
    io::Cursor,
    ptr::{copy, null_mut}
};


/// Get library version as C string.
#[no_mangle]
pub extern fn ssb_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Create renderer instance.
/// **script** mustn't be *null*.
/// **error_message** can be *null*.
#[no_mangle]
pub extern fn ssb_new_renderer(script: *const c_char, error_message: *mut c_char, error_message_capacity: c_ushort) -> *mut c_void {
    match ssb_new_renderer_inner(script) {
        Ok(renderer) => Box::into_raw(Box::new(renderer)) as *mut c_void,
        Err(err) => {
            if error_message != null_mut() && error_message_capacity > 0 {
                let mut msg = err.to_string();
                msg.truncate((error_message_capacity-1) as usize);
                msg.push('\0');
                unsafe {copy(msg.as_ptr() as *const c_char, error_message, msg.len());}
            }
            null_mut()
        }
    }
}
fn ssb_new_renderer_inner(script_file: *const c_char) -> Result<SsbRenderer, Box::<Error>> {
    Ok(SsbRenderer::new(
        Ssb::default().parse_owned( Cursor::new(unsafe{ CStr::from_ptr(script_file) }.to_str()?) )
        .and_then(|ssb| SsbRender::try_from(ssb) )?
    ))
}

/// Destroy renderer instance.
#[no_mangle]
pub extern fn ssb_destroy_renderer(renderer: *mut c_void) {
    if renderer != null_mut() {
        unsafe {Box::from_raw(renderer);}
    }
}