// Imports
use super::{
    error::*,
    data::*
};
use pest::Parser;
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use regex::*;


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
    pub struct Parser;
}

// RegEx pattern
lazy_static! {
    static ref MACRO_PATTERN: Regex = Regex::new("\\$\\{([a-zA-Z0-9_-]+)\\}").unwrap();
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
        // Flatten macros & detect infinite recursion
        let mut flat_macros = HashMap::with_capacity(self.data.macros.len());
        for macro_name in self.data.macros.keys() {
            if let Err(err) = flatten_macros(macro_name, &mut HashSet::new(), &self.data.macros, &mut flat_macros) {
                return Err(ParseError::new(&format!("Flattening macro '{}' caused error: {:?}", macro_name, err)));
            }
        }
        // Evaluate events
        let mut events = Vec::with_capacity(self.data.events.len());
        for event in &self.data.events {
            use ssb_event_data_peg::{Parser, Rule};
            // Insert base macro
            let mut event_data = event.data.clone();
            if let Some(macro_name) = &event.macro_ {
                event_data.insert_str(0, flat_macros.get(macro_name).ok_or_else(|| {
                    ParseError::new(&format!("Base macro '{}' not found to insert in event at line {}", macro_name, event.script_line))
                })?);
            }
            // Insert inline macros
            while let Some(found) = MACRO_PATTERN.find(&event_data) {
                let macro_name = &event_data[found.start()+2..found.end()-1];
                event_data.replace_range(
                    found.start()..found.end(),
                    flat_macros.get(macro_name).ok_or_else(|| {
                        ParseError::new(&format!("Inline macro '{}' not found to insert in event at line {}", macro_name, event.script_line))
                    })?
                );
            }

            // TODO: parse tags & geometries, pack into structure

            let _pairs = Parser::parse(Rule::events_data, &event.data).unwrap_or_else(|exception| {
                panic!("{}", exception);
            });
            events.push(
                EventRender {
                    trigger: event.trigger.clone(),
                    data: event_data
                }
            );
        }
        // Decode fonts
        let mut fonts = HashMap::with_capacity(self.data.fonts.len());
        for (font_face, font_data) in &self.data.fonts {
            fonts.insert(font_face.clone(), base64::decode(&font_data).map_err(|err| {
                ParseError::new(&format!("Base64 decoding of font '{}' failed: {}", font_face, err))
            })?);
        }
        // Decode textures
        let mut textures = HashMap::with_capacity(self.data.textures.len());
        for (texture_id, texture_data) in &self.data.textures {
            textures.insert(
                texture_id.clone(),
                match texture_data {
                    // Raw (base64)
                    TextureData::Raw(data) => {
                        base64::decode(data).map_err(|err| {
                            ParseError::new(&format!("Base64 decoding of texture '{}' failed: {}", texture_id, err))
                        })
                    }
                    // Url
                    TextureData::Url(path) => {
                        std::fs::read(path).map_err(|err| {
                            ParseError::new(&format!(
                                "File reading of texture '{}' in working directory '{}' failed: {}",
                                texture_id,
                                std::env::current_dir().map(|path| path.to_str().unwrap_or("").to_string()).unwrap_or("".to_string()),
                                err
                            ))
                        })
                    }
                }?
            );
        }
        // Return result
        Ok(SsbRender {
            target_width: self.data.target_width,
            target_height: self.data.target_height,
            target_depth: self.data.target_depth,
            target_view: self.data.target_view.clone(),
            events,
            fonts,
            textures
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
fn flatten_macros<'top>(macro_name: &str, history: &mut HashSet<String>, macros: &HashMap<String, String>, flat_macros: &mut HashMap<String, String>) -> Result<(), MacroError> {
    // Macro already flattened?
    if flat_macros.contains_key(macro_name) {
        return Ok(());
    }
    // Macro already in history (avoid infinite loop!)
    if history.contains(macro_name) {
        return Err(MacroError::InfiniteLoop(macro_name.to_string()));
    } else {
        history.insert(macro_name.to_string());
    }
    // Process macro value
    let mut flat_macro_value = macros.get(macro_name).ok_or(MacroError::NotFound(macro_name.to_string()))?.clone();
    while let Some(found) = MACRO_PATTERN.find(&flat_macro_value) {
        // Insert sub-macro
        let sub_macro_name = &flat_macro_value[found.start()+2..found.end()-1];
        if !flat_macros.contains_key(sub_macro_name) {
            flatten_macros(&sub_macro_name, history, macros, flat_macros)?;
        }
        flat_macro_value.replace_range(
            found.start()..found.end(),
            flat_macros.get(sub_macro_name).ok_or(MacroError::NotFound(sub_macro_name.to_string()))?
        );
    }
    // Register flat macro
    flat_macros.insert(
        macro_name.to_string(),
        flat_macro_value
    );
    // Everything alright
    Ok(())
}