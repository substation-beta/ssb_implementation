#[cfg(feature = "serialization")]
mod serialization_tests {
    // Test data
    #[derive(serde::Serialize)]
    struct Dummy {
        a: u8,
        b: f64
    }

    #[test]
    fn test_serde() {
        assert_eq!(
            serde_json::to_string(&Dummy {a: 8, b: 64.0}).expect("Dummy serialization must work!"),
            r#"{"a":8,"b":64.0}"#.to_owned()
        );
    }

    #[test]
    fn test_ssb() {
        use ssb_parser::data::Ssb;
        // Serialize
        let ssb_default = Ssb::default();
        let ssb_json = serde_json::to_string(&ssb_default).expect("Ssb serialization must work!");
        assert_eq!(
            ssb_json,
            r#"{"info_title":null,"info_author":null,"info_description":null,"info_version":null,"info_custom":{},"target_width":null,"target_height":null,"target_depth":1000,"target_view":"Perspective","macros":{},"events":[],"fonts":{},"textures":{}}"#.to_owned()
        );
        // Deserialize
        assert_eq!(
            serde_json::from_str::<Ssb>(&ssb_json).expect("Ssb deserialization must work!"),
            ssb_default
        );
    }
}