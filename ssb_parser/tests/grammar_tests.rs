mod grammar_tests {
    // Test 1
    mod simple {
        // Imports
        use pest_derive::Parser;    // Macro
        use pest::Parser;   // Trait

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
    }

    // Test 2
    mod ssb_alpha {
        const TEST_SCRIPT: &str =
"// Hello from comment!
# META
Title: MyTest
Author: Me

# FRAME
// Another comment
width: 1280
height: 720

# MACROS
# EVENTS
";

        // Tester
        #[test]
        fn test_ssb_alpha() {
            ssb_parser::processors::SsbParser::new(TEST_SCRIPT);
        }
    }
}