// Imports
use libc::*;


/// Get library version as string.
#[no_mangle]
pub extern fn version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}