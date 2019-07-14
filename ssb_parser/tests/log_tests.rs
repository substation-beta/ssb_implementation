mod log_tests {
    // Test data
    const LOG_TARGET: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/test.log");

    #[test]
    fn test_ssb_log() {
        // Initiale log file
        simple_logging::log_to_file(LOG_TARGET, log::LevelFilter::Debug).expect("Initialization of logging to file failed!");
        // Log by ssb parsing
        ssb_parser::data::Ssb::default().parse_owned(std::io::Cursor::new(
"#INFO
Title: Log test

#TARGET
Width: 1280
Height: 720"
        )).expect("Ssb parsing failed?!");
        // Read log file
        let log_content = std::fs::read_to_string(LOG_TARGET).expect("Reading log file failed!");
        let log_lines = log_content.lines().collect::<Vec<&str>>();
        // Check log outputs
        assert!(log_lines[0].ends_with("SSB parsing..."));
        assert!(log_lines[1].ends_with("0: #INFO"));
        assert!(log_lines[2].ends_with("1: Title: Log test"));
        assert!(log_lines[3].ends_with("2: "));
        assert!(log_lines[4].ends_with("3: #TARGET"));
        assert!(log_lines[5].ends_with("4: Width: 1280"));
        assert!(log_lines[6].ends_with("5: Height: 720"));
    }
}