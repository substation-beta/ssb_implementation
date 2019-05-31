mod grammar_tests {
    // Imports
    use ssb_parser::{
        //types::EventTrigger,
        data::{Ssb, SsbRender}
    };
    use std::{
        convert::TryFrom,
        io::Cursor,
        path::Path
    };


    // Tester
    #[test]
    fn test_ssb() {
        // Parse 1st phase
        let mut ssb = Ssb::default();
        ssb.parse(
            Cursor::new(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.ssb"))),
            Some(Path::new(env!("CARGO_MANIFEST_DIR")))
        ).unwrap_or_else(|exception| {
            panic!("SSB parsing error: {}", exception)
        });
        //assert_eq!(ssb.info_title.as_ref().expect("Info title should be available!"), "test");
        println!("{:?}", ssb);
        // Parse 2nd phase
        let ssb_render = SsbRender::try_from(ssb).unwrap_or_else(|exception| {
            panic!("SSB render data error: {}", exception)
        });
        //assert_eq!(ssb_render.events.get(0).expect("First event missing!").trigger, EventTrigger::Time((2000, 300000)));
        println!("{:?}", ssb_render);
    }
}