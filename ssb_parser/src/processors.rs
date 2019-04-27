// Imports
use super::{
    error::ParseError,
    data::*
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
            match section_entry_pair.as_rule() {
                // Info entries
                Rule::info_entry => {
                    if let Some(info_entry_pair) = section_entry_pair.into_inner().next() {
                        match info_entry_pair.as_rule() {
                            // Title
                            Rule::info_title_value => self.data.info_title = Some(info_entry_pair.as_str().to_string()),
                            // Author
                            Rule::info_author_value => self.data.info_author = Some(info_entry_pair.as_str().to_string()),
                            // Description
                            Rule::info_desc_value => self.data.info_description = Some(info_entry_pair.as_str().to_string()),
                            // Version
                            Rule::info_version_value => self.data.info_version = Some(info_entry_pair.as_str().to_string()),
                            // Custom
                            Rule::info_custom_entry => {
                                let mut info_custom_entry_pair = info_entry_pair.into_inner();
                                if let (Some(info_custom_key), Some(info_custom_value)) = (info_custom_entry_pair.next(), info_custom_entry_pair.next()) {
                                    self.data.info_custom.insert(info_custom_key.as_str().to_string(), info_custom_value.as_str().to_string());
                                }
                            }
                            _ => ()
                        }
                    }
                }
                // Target entries
                Rule::target_entry => {
                    if let Some(target_entry_pair) = section_entry_pair.into_inner().next() {
                        match target_entry_pair.as_rule() {
                            // Width
                            Rule::target_width_value => if let Ok(width) = target_entry_pair.as_str().parse::<u16>() {
                                self.data.target_width = Some(width);
                            }
                            // Height
                            Rule::target_height_value => if let Ok(height) = target_entry_pair.as_str().parse::<u16>() {
                                self.data.target_height = Some(height);
                            }
                            // Depth
                            Rule::target_depth_value => if let Ok(depth) = target_entry_pair.as_str().parse::<u16>() {
                                self.data.target_depth = depth;
                            }
                            // View
                            Rule::target_view_value => if let Ok(view) = View::from_str(target_entry_pair.as_str()) {
                                self.data.target_view = view;
                            }
                            _ => ()
                        }
                    }
                }
                // Macros entries
                Rule::macros_entry => {
                    let mut macros_entry_pair = section_entry_pair.into_inner();
                    if let (Some(macros_key), Some(macros_value)) = (macros_entry_pair.next(), macros_entry_pair.next()) {
                        self.data.macros.insert(macros_key.as_str().to_string(), macros_value.as_str().to_string());
                    }
                }
                // Events entries
                Rule::events_entry => {
                    let mut events_entry_pair = section_entry_pair.into_inner();
                    if let (Some(events_trigger), Some(events_macro), Some(events_note), Some(events_data))
                        = (events_entry_pair.next(), events_entry_pair.next(), events_entry_pair.next(), events_entry_pair.next()) {
                        self.data.events.push(Event {
                            script_line: events_trigger.as_span().start_pos().line_col().0,
                            trigger: match events_trigger.as_rule() {
                                Rule::events_id => EventTrigger::Id(events_trigger.as_str().to_string()),
                                Rule::events_time => EventTrigger::Id("TODO".to_string()),  // TODO: implement correctly
                                _ => EventTrigger::Id("".to_string())
                            },
                            macro_: Some(events_macro.as_str().to_string()),
                            note: Some(events_note.as_str().to_string()),
                            data: events_data.as_str().to_string()
                        });
                    }
                }
                // Resources entries
                Rule::resources_entry => {
                    if let Some(resources_entry_pair) = section_entry_pair.into_inner().next() {
                        match resources_entry_pair.as_rule() {
                            // Font
                            Rule::resources_font_entry => {
                                // TODO: implement
                            }
                            // Texture
                            Rule::resources_texture_entry => {
                                // TODO: implement
                            }
                            _ => ()
                        }
                    }
                }
                // Unrelevant matches
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