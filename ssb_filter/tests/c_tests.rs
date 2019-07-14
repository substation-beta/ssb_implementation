mod c_tests {
    // Imports
    use libloading::Library;
    use libc::*;
    use std::{
        ffi::{CStr, CString},
        ptr::null_mut
    };
    include!("platform.irs");    // Tests are separated, thus include code (::dll_path)

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
        let lib = Library::new(platform::dll_path()).expect("Couldn't load DLL!");
        unsafe {
            let new_renderer_fn = lib.get::<unsafe extern fn(*const c_char, *mut c_char, c_ushort) -> *mut c_void>(b"ssb_new_renderer\0").expect("Couldn't load symbol 'ssb_new_renderer' from DLL!");
            let destroy_renderer_fn = lib.get::<unsafe extern fn(*mut c_void)>(b"ssb_destroy_renderer\0").expect("Couldn't load symbol 'ssb_destroy_renderer' from DLL!");
            if let Ok(script) = CString::new("#Events\n0-1.|||") {
                let renderer = new_renderer_fn(script.as_ptr(), null_mut(), 0);
                assert_ne!(renderer, null_mut());
                destroy_renderer_fn(renderer);
            }
        }
    }
}