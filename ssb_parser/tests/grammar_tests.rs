mod grammar_tests {
    // Imports
    use pest_derive::Parser;    // Macro
    use pest::Parser;   // Trait
    use ssb_parser::{
        data::EventTrigger,
        processing::SsbParser
    };
    use std::path::Path;

    // Test resource
    #[derive(Parser)]
    #[grammar = "../tests/simple.pest"]
    struct IdentParser;

    // Tester
    #[test]
    fn test_simple_pest() {
        // Parse text by grammer and fill structures
        let pairs = IdentParser::parse(Rule::ident_list, "a1 b2").unwrap_or_else(|e| panic!("{}", e));
        // Because ident_list is silent, the iterator will contain idents
        for pair in pairs {
            let span = pair.clone().as_span();
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", span);
            println!("Text:    {}", span.as_str());
            // A pair can be converted to an iterator of the tokens which make it up:
            for inner_pair in pair.into_inner() {
                let inner_span = inner_pair.clone().as_span();
                match inner_pair.as_rule() {
                    Rule::alpha => println!("Letter:  {}", inner_span.as_str()),
                    Rule::digit => println!("Digit:   {}", inner_span.as_str()),
                    _ => unreachable!()
                };
            }
        }
    }

    #[test]
    fn test_ssb() {
        // Parse 1st phase
        let parser = SsbParser::new(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.ssb"))).unwrap_or_else(|exception| {
            panic!("SSB parsing error: {}", exception)
        });
        let data = parser.data();
        assert_eq!(data.info_title.as_ref().expect("Info title should be available!"), "test");
        // Show data
        println!("{:?}", data);
        // Parse 2nd phase
        let render_data = parser.render_data(
            Some(Path::new(env!("CARGO_MANIFEST_DIR")))
        ).unwrap_or_else(|exception| {
            panic!("SSB parsing error: {}", exception)
        });
        assert_eq!(render_data.events.get(0).expect("First event missing!").trigger, EventTrigger::Time((2000, 300000)));
        // Show render data
        println!("{:?}", render_data);
    }
}