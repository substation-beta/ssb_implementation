#[cfg(test)]
mod gl_utils_tests {
    // Imports
    use ssb_renderer::gl_utils::{error::GlError, safe::*, environment::GlEnvironment};
    use std::sync::mpsc::channel;

    // Test resource
    fn do_gl_things(data: u8) -> u8 {
        assert!(data == 42 || data == 9);
        // Note: GL version string may not contain current profile but newest possible version (f.e. mesa)
        assert!(GetString(gl32::VERSION).is_some());
        // Zero is always an invalid ID in OpenGL
        assert!(GetString(0).is_none());
        println!("{}", GlError::from_gl().expect("Last GL call should have been wrong!"));
        assert!(GlError::from_gl().is_none());
        data + 1
    }

    // Tester
    #[test]
    fn test_gl_environment() {
        let gl_env = GlEnvironment::new((3, 2), do_gl_things);
        assert!(gl_env.process(42).expect("Simple process didn't work!") == 43);
        assert!(gl_env.process(9).expect("Another simple process didn't work!") == 10);
    }

    // Tester
    #[test]
    fn test_gl_error() {
        // Send error
        let (sender, _) = channel::<u8>();
        let send_err = GlError::from(sender.send(0).expect_err("Sending shouldn't be possible!"));
        println!("{}", send_err);
        // Receive error
        let (_, receiver) = channel::<u8>();
        let recv_err = GlError::from(receiver.recv().expect_err("Receiving shouldn't be possible!"));
        println!("{:?}", recv_err);
    }
}