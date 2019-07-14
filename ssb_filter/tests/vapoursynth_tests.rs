#[cfg(feature = "vapoursynth-interface")]
mod vapoursynth_tests {
    // Imports
    use vapoursynth::prelude::*;
    use std::process::Command;
    include!("platform.irs");    // Tests are separated, thus include code

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
        let output = Command::new(platform::constants::PYTHON_CMD)
            .arg("--version")
            .output().expect("Couldn't find python!");
        // Python 3 at least required
        assert!(String::from_utf8_lossy(&output.stdout).to_string().contains("Python 3."));
    }

    #[test]
    fn test_plugin_load() {
        // Load plugin with vapoursynth by python execution
        let output = Command::new(platform::constants::PYTHON_CMD)
            .arg("-c")
            .arg(format!(
                include_str!("test.vpy"),
                platform::dll_path()
            ))
            .output().expect("Couldn't load vapoursynth plugin!");
        // No output on standard error stream -> success!
        assert!(output.stderr.is_empty(), "Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}