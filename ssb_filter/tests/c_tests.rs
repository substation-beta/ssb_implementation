mod c_tests {
    // Imports
    use libloading::Library;
    use libc::*;
    use std::{
        ffi::CStr,
        ptr::null_mut
    };
    include!("platform.irs");    // Tests are separated, thus include code

    #[test]
    fn test_version() {
        let lib = Library::new(platform::dll_path()).expect("Couldn't load DLL!");
        unsafe {
            let version_fn = lib.get::<unsafe extern fn() -> *const c_char>(b"ssb_version\0").expect("Couldn't load symbol 'ssb_version' from DLL!");
            assert_eq!(
                CStr::from_ptr(version_fn()).to_string_lossy(),
                env!("CARGO_PKG_VERSION")
            );
        }
    }

    #[test]
    fn test_renderer() {
        // Get DLL functions
        let lib = Library::new(platform::dll_path()).expect("Couldn't load DLL!");
        unsafe {
            let new_renderer_fn = lib.get::<unsafe extern fn(*const c_char, *mut c_char, c_ushort) -> *mut c_void>(b"ssb_new_renderer\0").expect("Couldn't load symbol 'ssb_new_renderer' from DLL!");
            let destroy_renderer_fn = lib.get::<unsafe extern fn(*mut c_void)>(b"ssb_destroy_renderer\0").expect("Couldn't load symbol 'ssb_destroy_renderer' from DLL!");
            let render_fn = lib.get::<unsafe extern fn(*mut c_void, c_ushort, c_ushort, c_uint, *const c_char, *const *mut c_uchar, c_uint, *mut c_char, c_ushort) -> c_int>(b"ssb_render\0").expect("Couldn't load symbol 'ssb_render' from DLL!");
            // Try rendering
            let renderer = new_renderer_fn(
                "#Events\n0-1.|||\0".as_ptr() as *const c_char,
                null_mut(), 0
            );
            assert_ne!(renderer, null_mut());
            assert_eq!(
                render_fn(
                    renderer,
                    640, 480, 640*3,
                    "RGB24\0".as_ptr() as *const c_char,
                    vec![vec![0u8;640*480*3]].iter_mut().map(|plane| plane.as_mut_ptr() ).collect::<Vec<_>>().as_ptr(),
                    1000,
                    null_mut(), 0
                ),
                0
            );
            destroy_renderer_fn(renderer);
            // Error case
            let mut error_message = vec![0u8;128];
            assert_eq!(
                new_renderer_fn(
                    "INVALID\0".as_ptr() as *const c_char,
                    error_message.as_mut_ptr() as *mut c_char, error_message.len() as c_ushort
                ),
                null_mut()
            );
            assert_eq!(
                CStr::from_ptr(error_message.as_ptr() as *const c_char).to_string_lossy(),
                "No section set! <0:0>"
            );
        }
    }
}