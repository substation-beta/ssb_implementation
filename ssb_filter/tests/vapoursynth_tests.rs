#[cfg(feature = "vapoursynth-interface")]
mod vapoursynth_tests {
    // Imports
    use vapoursynth::prelude::*;
    use std::process::Command;
    use std::path::Path;

    // Platform properties
    #[cfg(target_os = "windows")]
    mod platform {
        pub const PYTHON_CMD: &str = "python";
        pub const LIB_PREFIX: &str = "";
        pub const LIB_EXTENSION: &str = ".dll";
    }
    #[cfg(target_os = "linux")]
    mod platform {
        pub const PYTHON_CMD: &str = "python3";
        pub const LIB_PREFIX: &str = "lib";
        pub const LIB_EXTENSION: &str = ".so";
    }
    #[cfg(target_os = "macos")]
    mod platform {
        pub const PYTHON_CMD: &str = "python3";
        pub const LIB_PREFIX: &str = "lib";
        pub const LIB_EXTENSION: &str = ".dylib";
    }

    #[test]
    fn test_core_available() {
        // Create scripting environment
        let environment = Environment::new().expect("Couldn't create a VSScript environment!");
        // Get core functions
        let core = environment.get_core().expect("Couldn't create the VapourSynth core!");
        // Output version
        println!("Core version: {}", core.info().version_string);
    }

    #[test]
    fn test_python_available() {
        // Get python version
        let output = Command::new(platform::PYTHON_CMD)
            .arg("--version")
            .output().expect("Couldn't find python!");
        // Python 3 at least required
        assert!(String::from_utf8_lossy(&output.stdout).to_string().contains("Python 3."));
    }

    #[test]
    fn test_plugin_load() {
        // Path to compiled plugin
        let plugin_path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../target/",
            env!("PROFILE") // Set by build script
        )).join(
            platform::LIB_PREFIX.to_string() +
            &env!("CARGO_PKG_NAME") +
            platform::LIB_EXTENSION
        );
        // Load plugin with vapoursynth by python execution
        let output = Command::new(platform::PYTHON_CMD)
            .arg("-c")
            .arg(format!(
                concat!(
                    "from vapoursynth import core;",
                    "core.std.LoadPlugin({:?});",
                    "core.ssb.render(core.std.BlankClip(None, 100, 100), \"test.ssb\").get_frame(0)"
                ),
                plugin_path
            ))
            .output().expect("Couldn't load vapoursynth plugin!");
        // No output on standard error stream -> success!
        assert!(output.stderr.is_empty(), "Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}