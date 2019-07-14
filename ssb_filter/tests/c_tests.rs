mod c_tests {
    // Imports
    use libloading::Library;
    use libc::*;
    use std::ffi::CStr;
    include!("platform.irs");    // Tests are separated, thus include code (::dll_path)

    #[test]
    fn test_version() {
        let lib = Library::new(dll_path()).expect("Couldn't load DLL!");
        unsafe {
            let version_fn = lib.get::<unsafe extern fn() -> *const c_char>(b"version\0").expect("Couldn't load symbol 'version' from DLL!");
            assert_eq!(
                CStr::from_ptr(version_fn()).to_string_lossy(),
                env!("CARGO_PKG_VERSION")
            );
        }
    }
}