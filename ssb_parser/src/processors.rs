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
                                    self.data.info_custom.insert(
                                        info_custom_key.as_str().to_string(),
                                        info_custom_value.as_str().to_string()
                                    );
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
                    let mut macros_entry_pairs = section_entry_pair.into_inner();
                    if let (Some(macros_key), Some(macros_value)) = (macros_entry_pairs.next(), macros_entry_pairs.next()) {
                        self.data.macros.insert(
                            macros_key.as_str().to_string(),
                            macros_value.as_str().to_string()
                        );
                    }
                }
                // Events entries
                Rule::events_entry => {
                    let mut events_entry_pairs = section_entry_pair.into_inner();
                    if let (Some(events_trigger), Some(events_macro), Some(events_note), Some(events_data))
                        = (events_entry_pairs.next(), events_entry_pairs.next(), events_entry_pairs.next(), events_entry_pairs.next()) {
                        // Add event
                        self.data.events.push(
                            Event {
                                // Script line
                                script_line: events_trigger.as_span().start_pos().line_col().0,
                                // Events trigger
                                trigger: match events_trigger.as_rule() {
                                    // Id
                                    Rule::events_id => EventTrigger::Id(events_trigger.as_str().to_string()),
                                    // Time
                                    Rule::events_time => {
                                        let mut time = (0, 0);
                                        let mut events_time_pairs = events_trigger.into_inner();
                                        if let (Some(events_start_time), Some(events_end_time)) = (events_time_pairs.next(), events_time_pairs.next()) {
                                            // Start time
                                            for events_start_time_pair in events_start_time.into_inner() {
                                                if let Ok(unit) = events_start_time_pair.as_str().parse::<u32>() {
                                                    time.0 += unit * rule_to_ms(events_start_time_pair.as_rule());
                                                }
                                            }
                                            // End time
                                            for events_end_time_pair in events_end_time.into_inner() {
                                                if let Ok(unit) = events_end_time_pair.as_str().parse::<u32>() {
                                                    time.1 += unit * rule_to_ms(events_end_time_pair.as_rule());
                                                }
                                            }
                                        }
                                        EventTrigger::Time(time)
                                    },
                                    _ => EventTrigger::Id("".to_string())
                                },
                                // Events macro
                                macro_: Some(events_macro.as_str().to_string()).filter(|s| !s.is_empty()),
                                // Events note
                                note: Some(events_note.as_str().to_string()).filter(|s| !s.is_empty()),
                                // Events data
                                data: events_data.as_str().to_string()
                            }
                        );
                    }
                }
                // Resources entries
                Rule::resources_entry => {
                    if let Some(resources_entry_pair) = section_entry_pair.into_inner().next() {
                        match resources_entry_pair.as_rule() {
                            // Font
                            Rule::resources_font_entry => {
                                let mut resources_font_entry_pairs = resources_entry_pair.into_inner();
                                if let (Some(resources_font_family), Some(resources_font_style), Some(resources_font_data))
                                    = (resources_font_entry_pairs.next(), resources_font_entry_pairs.next(), resources_font_entry_pairs.next()) {
                                    // Add font
                                    self.data.fonts.insert(
                                        FontFace {
                                            family: resources_font_family.as_str().to_string(),
                                            style: FontStyle::from_str(resources_font_style.as_str()).unwrap_or(FontStyle::Regular)
                                        },
                                        resources_font_data.as_str().to_string()
                                    );
                                }
                            }
                            // Texture
                            Rule::resources_texture_entry => {
                                let mut resources_texture_entry_pairs = resources_entry_pair.into_inner();
                                if let(Some(resources_texture_id), Some(resources_texture_data))
                                    = (resources_texture_entry_pairs.next(), resources_texture_entry_pairs.next()) {
                                    // Add texture
                                    self.data.textures.insert(
                                        resources_texture_id.as_str().to_string(),
                                        match resources_texture_data.as_rule() {
                                            Rule::resources_texture_data_url => TextureData::Url(resources_texture_data.as_str().to_string()),
                                            Rule::resources_texture_data_raw => TextureData::Raw(resources_texture_data.as_str().to_string()),
                                            _ => TextureData::Url("".to_string())
                                        }
                                    );
                                }
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

    // Generate data relevant for rendering
    pub fn render_data(&self) -> Result<SsbRender, ParseError> {
        use std::collections::HashMap;
        Ok(SsbRender {
            target_width: self.data.target_width,
            target_height: self.data.target_height,
            target_depth: self.data.target_depth,
            target_view: self.data.target_view.clone(),
            // TODO: convert correctly below
            events: vec!(),
            fonts: HashMap::new(),
            textures: HashMap::new()
            /*
            events: self.data.events.into_iter().map(|entry| {
                use ssb_event_data_peg::{Parser, Rule};
                // Parse script and throw possible error
                let _pairs = Parser::parse(Rule::events_data, &entry.data).unwrap_or_else(|exception| {
                    panic!("{}", exception);
                });
                EventRender {
                    trigger: entry.trigger,
                    data: entry.data
                }
            }).collect(),
            fonts: self.data.fonts.into_iter().map(|(key, value)| {
                (key, value.into_bytes())
            }).collect(),
            textures: self.data.textures.into_iter().map(|(key, value)| {
                (key, format!("{:?}", value).into_bytes())
            }).collect()
            */
        })
    }
}

// Helpers
#[inline]
fn rule_to_ms(rule: ssb_peg::Rule) -> u32 {
    match rule {
        ssb_peg::Rule::time_ms => 1,
        ssb_peg::Rule::time_s => 1000,
        ssb_peg::Rule::time_m => 60 * 1000,
        ssb_peg::Rule::time_h => 60 * 60 * 1000,
        _ => 0
    }
}