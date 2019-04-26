// Imports
use super::{
    error::ParseError,
    data::Ssb
};
use pest::Parser;

// PEG parsers
mod ssb_peg{
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "ssb.pest"]
    pub struct Parser;
}
mod ssb_event_data_peg {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "ssb_event_data.pest"]
    pub struct _Parser;
}

// Stream parser for ssb data
pub struct SsbParser {
    _data: Ssb
}
impl Default for SsbParser {
    fn default() -> Self {
        Self {
            _data: Ssb::default()
        }
    }
}
impl SsbParser {
    // Constructor
    pub fn new(script: &str) -> Result<Self, ParseError> {
        let mut instance = Self::default();
        instance.parse(script)?;
        Ok(instance)
    }

    // Parsing / modifying instance
    pub fn parse(&mut self, script: &str) -> Result<&mut Self, ParseError> {
        // Parse script and panic on fail
        let _pairs = ssb_peg::Parser::parse(ssb_peg::Rule::script, script)?;
        /*
        // Iterate through section entries
        for section_entry_pair in pairs {
            match section_entry_pair.as_rule() {
                // Meta entry
                Rule::meta_entry => for meta_entry_pair in section_entry_pair.into_inner() {
                    match meta_entry_pair.as_rule() {
                        // Meta entry key
                        Rule::meta_entry_key => {
                            println!("Meta key: {}", meta_entry_pair.as_str());
                        }
                        // Meta entry value
                        Rule::meta_entry_value => {
                            println!("Meta value: {}", meta_entry_pair.as_str());
                        }
                        // Nothing more in this scope
                        _ => unreachable!()
                    }
                }
                // Frame entry
                Rule::frame_entry => for frame_entry_pair in section_entry_pair.into_inner() {
                    match frame_entry_pair.as_rule() {
                        // Frame entry key
                        Rule::frame_entry_key => {
                            println!("Frame key: {}", frame_entry_pair.as_str());
                        }
                        // Frame entry value
                        Rule::frame_entry_value => {
                            println!("Frame value: {}", frame_entry_pair.as_str());
                        }
                        // Nothing more in this scope
                        _ => unreachable!()
                    }
                }
                // "End of input" not of interest
                Rule::EOI => (),
                // Nothing more in this scope
                _ => unreachable!()
            }
        }
        */
        // Pass instance further
        Ok(self)
    }
}