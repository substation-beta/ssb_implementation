// Imports
use super::{
    error::ParseError,
    data::{
        Ssb,
        View
    }
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
    data: Ssb
}
impl Default for SsbParser {
    fn default() -> Self {
        Self {
            data: Ssb::default()
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
        use ssb_peg::{Parser, Rule};
        // Parse script and throw possible error
        let pairs = Parser::parse(Rule::script, script)?;
        // Iterate through section entries
        for section_entry_pair in pairs {
            println!("{:?}", section_entry_pair.as_rule());
            match section_entry_pair.as_rule() {
                // Info entries
                Rule::info_title_value => self.data.info_title = Some(section_entry_pair.as_span().as_str().to_string()),
                Rule::info_author_value => self.data.info_author = Some(section_entry_pair.as_span().as_str().to_string()),
                Rule::info_desc_value => self.data.info_description = Some(section_entry_pair.as_span().as_str().to_string()),
                Rule::info_version_value => self.data.info_version = Some(section_entry_pair.as_span().as_str().to_string()),
                Rule::info_custom_entry => {
                    let mut info_custom_entry_pair = section_entry_pair.into_inner();
                    if let (Some(info_custom_key), Some(info_custom_value)) = (info_custom_entry_pair.next(), info_custom_entry_pair.next()) {
                        self.data.info_custom.insert(info_custom_key.as_str().to_string(), info_custom_value.as_str().to_string());
                    }
                }
                // Target entries
                Rule::target_width_value => if let Ok(width) = section_entry_pair.as_span().as_str().parse::<u16>() {
                    self.data.target_width = Some(width);
                }
                Rule::target_height_value => if let Ok(height) = section_entry_pair.as_span().as_str().parse::<u16>() {
                    self.data.target_height = Some(height);
                }
                Rule::target_depth_value => if let Ok(depth) = section_entry_pair.as_span().as_str().parse::<u16>() {
                    self.data.target_width = Some(depth);
                }
                Rule::target_view_value => if let Ok(view) = View::from_str(section_entry_pair.as_span().as_str()) {
                    self.data.target_view = view;
                }
                // Macros entries
                Rule::macros_entry => {
                    let mut macros_entry_pair = section_entry_pair.into_inner();
                    if let (Some(macros_key), _, Some(macros_value)) = (macros_entry_pair.next(), macros_entry_pair.next(), macros_entry_pair.next()) {
                        self.data.macros.insert(macros_key.as_str().to_string(), macros_value.as_str().to_string());
                    }
                }
                // Events entries
                // TODO: implement rest below
                Rule::events_entry => (),
                // Resources entries
                Rule::resources_font_entry => (),
                Rule::resources_texture_entry => (),
                // Unrelevant catches
                _ => ()
            }
        }
        // Pass instance further
        Ok(self)
    }

    // View on internal data
    pub fn data(&self) -> &Ssb {
        &self.data
    }
}