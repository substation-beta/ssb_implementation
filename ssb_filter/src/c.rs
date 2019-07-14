// Imports
use libc::*;


/// Get library version as C string.
#[no_mangle]
pub extern fn ssb_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}