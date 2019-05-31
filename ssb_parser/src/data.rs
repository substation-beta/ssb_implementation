// Imports
use super::{
    error::*,
    types::*
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap,HashSet},
    io::BufRead,
    path::Path,
    convert::TryFrom
};


// Raw data
#[derive(Debug)]
pub struct Ssb {
    // Info section
    pub info_title: Option<String>,
    pub info_author: Option<String>,
    pub info_description: Option<String>,
    pub info_version: Option<String>,
    pub info_custom: HashMap<String, String>,
    // Target section
    pub target_width: Option<u16>,
    pub target_height: Option<u16>,
    pub target_depth: u16,
    pub target_view: View,
    // Macros section
    pub macros: HashMap<String, String>,
    // Events section
    pub events: Vec<Event>,
    // Resources section
    pub fonts: HashMap<FontFace, FontData>,
    pub textures: HashMap<TextureId, TextureData>
}
impl Default for Ssb {
    fn default() -> Self {
        Self {
            info_title: None,
            info_author: None,
            info_description: None,
            info_version: None,
            info_custom: HashMap::default(),
            target_width: None,
            target_height: None,
            target_depth: 1000,
            target_view: View::Perspective,
            macros: HashMap::default(),
            events: Vec::default(),
            fonts: HashMap::default(),
            textures: HashMap::default()
        }
    }
}
impl Ssb {
    pub fn parse<R>(&mut self, reader: R, search_path: Option<&Path>) -> Result<&mut Self, ParseError> 
        where R: BufRead {
        // Initial state
        let mut section: Option<Section> = None;
        // Iterate through text lines
        for (line_index, line) in reader.lines().enumerate() {
            // Check for valid UTF-8 and remove carriage return (leftover of windows-ending)
            let mut line = line?;
            if line.ends_with("\r") {
                line.pop();
            }
            // Ignore empty lines & comments
            if !(line.is_empty() || line.starts_with("//")) {
                // Switch or handle section
                if let Ok(parsed_section) = Section::try_from(line.as_ref()) {
                    section = Some(parsed_section);
                } else {
                    match section {
                        // Info section
                        Some(Section::Info) => {
                            // Title
                            if line.starts_with(INFO_TITLE_KEY) {
                                self.info_title = Some(line[INFO_TITLE_KEY.len()..].to_owned());
                            }
                            // Author
                            else if line.starts_with(INFO_AUTHOR_KEY) {
                                self.info_author = Some(line[INFO_AUTHOR_KEY.len()..].to_owned());
                            }
                            // Description
                            else if line.starts_with(INFO_DESCRIPTION_KEY) {
                                self.info_description = Some(line[INFO_DESCRIPTION_KEY.len()..].to_owned());
                            }
                            // Version
                            else if line.starts_with(INFO_VERSION_KEY) {
                                self.info_version = Some(line[INFO_VERSION_KEY.len()..].to_owned());
                            }
                            // Custom
                            else if let Some(separator_pos) = line.find(KEY_SUFFIX) {
                                self.info_custom.insert(
                                    line[..separator_pos].to_owned(),
                                    line[separator_pos + KEY_SUFFIX.len()..].to_owned()
                                );
                            }
                            // Invalid entry
                            else {
                                return Err(ParseError::new_with_pos("Invalid info entry!", (line_index, 0)));
                            }
                        }
                        // Target section
                        Some(Section::Target) => {
                            // Width
                            if line.starts_with(TARGET_WIDTH_KEY) {
                                if let Ok(width) = line[TARGET_WIDTH_KEY.len()..].parse() {
                                    self.target_width = Some(width);
                                } else {
                                    return Err(ParseError::new_with_pos("Invalid target width value!", (line_index, TARGET_WIDTH_KEY.len())));
                                }
                            }
                            // Height
                            else if line.starts_with(TARGET_HEIGHT_KEY) {
                                if let Ok(height) = line[TARGET_HEIGHT_KEY.len()..].parse() {
                                    self.target_height = Some(height);
                                } else {
                                    return Err(ParseError::new_with_pos("Invalid target height value!", (line_index, TARGET_HEIGHT_KEY.len())));
                                }
                            }
                            // Depth
                            else if line.starts_with(TARGET_DEPTH_KEY) {
                                if let Ok(depth) = line[TARGET_DEPTH_KEY.len()..].parse() {
                                    self.target_depth = depth;
                                } else {
                                    return Err(ParseError::new_with_pos("Invalid target depth value!", (line_index, TARGET_DEPTH_KEY.len())));
                                }
                            }
                            // View
                            else if line.starts_with(TARGET_VIEW_KEY) {
                                if let Ok(view) = View::try_from(&line[TARGET_VIEW_KEY.len()..]) {
                                    self.target_view = view;
                                } else {
                                    return Err(ParseError::new_with_pos("Invalid target view value!", (line_index, TARGET_VIEW_KEY.len())));
                                }
                            }
                            // Invalid entry
                            else {
                                return Err(ParseError::new_with_pos("Invalid target entry!", (line_index, 0)));
                            }
                        }
                        // Macros section
                        Some(Section::Macros) => {
                            // Macro
                            if let Some(separator_pos) = line.find(KEY_SUFFIX) {
                                self.macros.insert(
                                    line[..separator_pos].to_owned(),
                                    line[separator_pos + KEY_SUFFIX.len()..].to_owned()
                                );
                            }
                            // Invalid entry
                            else {
                                return Err(ParseError::new_with_pos("Invalid macros entry!", (line_index, 0)));
                            }
                        }
                        // Events section
                        Some(Section::Events) => {

                            // TODO

                        }
                        // Resources section
                        Some(Section::Resources) => {
                            // Font
                            if line.starts_with(RESOURCES_FONT_KEY) {

                            }
                            // Texture
                            else if line.starts_with(RESOURCES_TEXTURE_KEY) {

                            }
                            // Invalid entry
                            else {
                                return Err(ParseError::new_with_pos("Invalid resources entry!", (line_index, 0)));
                            }
                        }
                        // Unset section
                        None => return Err(ParseError::new_with_pos("Set section first!", (line_index, 0)))
                    }
                }
            }
        }
        // Return self for chaining calls
        Ok(self)
    }
}


// Processed data (for rendering)
#[derive(Debug)]
pub struct SsbRender {
    // Target section
    pub target_width: Option<u16>,
    pub target_height: Option<u16>,
    pub target_depth: u16,
    pub target_view: View,
    // Events section
    pub events: Vec<EventRender>,
    // Resources section
    pub fonts: HashMap<FontFace, FontData>,
    pub textures: HashMap<TextureId, TextureData>
}
impl TryFrom<Ssb> for SsbRender {
    type Error = ParseError;

    fn try_from(data: Ssb) -> Result<Self, Self::Error> {
        // Flatten macros & detect infinite recursion
        let mut flat_macros = HashMap::with_capacity(data.macros.len());
        for macro_name in data.macros.keys() {
            if let Err(err) = flatten_macro(macro_name, &mut HashSet::new(), &data.macros, &mut flat_macros) {
                return Err(ParseError::new(&format!("Flattening macro '{}' caused error: {:?}", macro_name, err)));
            }
        }
        // Evaluate events
        let mut events = Vec::with_capacity(data.events.len());
        for event in &data.events {
            // Insert base macro
            let mut event_data = event.data.clone();
            if let Some(macro_name) = &event.macro_name {
                event_data.insert_str(0, flat_macros.get(macro_name).ok_or_else(|| {
                    ParseError::new(&format!("Base macro '{}' not found to insert in event at line {}", macro_name, event.data_location.0))
                })?);
            }
            // Insert inline macros
            while let Some(found) = MACRO_PATTERN.find(&event_data) {
                let macro_name = &event_data[found.start()+2..found.end()-1];
                event_data.replace_range(
                    found.start()..found.end(),
                    flat_macros.get(macro_name).ok_or_else(|| {
                        ParseError::new(&format!("Inline macro '{}' not found to insert in event at line {}", macro_name, event.data_location.0))
                    })?
                );
            }
            // Collect event objects by line tokens
            let objects = vec!();
            let _mode = Mode::default();

            // TODO

            // Save event for rendering
            events.push(
                EventRender {
                    trigger: event.trigger.clone(),
                    objects
                }
            );
        }
        // Return result
        Ok(SsbRender {
            target_width: data.target_width,
            target_height: data.target_height,
            target_depth: data.target_depth,
            target_view: data.target_view,
            events,
            fonts: data.fonts,
            textures: data.textures
        })
    }
}


// Helpers
const INFO_TITLE_KEY: &str = "Title: ";
const INFO_AUTHOR_KEY: &str = "Author: ";
const INFO_DESCRIPTION_KEY: &str = "Description: ";
const INFO_VERSION_KEY: &str = "Version: ";
const KEY_SUFFIX: &str = ": ";
const TARGET_WIDTH_KEY: &str = "Width: ";
const TARGET_HEIGHT_KEY: &str = "Height: ";
const TARGET_DEPTH_KEY: &str = "Depth: ";
const TARGET_VIEW_KEY: &str = "View: ";
const RESOURCES_FONT_KEY: &str = "Font: ";
const RESOURCES_TEXTURE_KEY: &str = "Texture: ";
lazy_static! {
    static ref MACRO_PATTERN: Regex = Regex::new("\\$\\{([a-zA-Z0-9_-]+)\\}").unwrap();
}

fn flatten_macro(macro_name: &str, history: &mut HashSet<String>, macros: &HashMap<String, String>, flat_macros: &mut HashMap<String, String>) -> Result<(), MacroError> {
    // Macro already flattened?
    if flat_macros.contains_key(macro_name) {
        return Ok(());
    }
    // Macro already in history (avoid infinite loop!)
    if history.contains(macro_name) {
        return Err(MacroError::InfiniteLoop(macro_name.to_owned()));
    } else {
        history.insert(macro_name.to_owned());
    }
    // Process macro value
    let mut flat_macro_value = macros.get(macro_name).ok_or(MacroError::NotFound(macro_name.to_owned()))?.clone();
    while let Some(found) = MACRO_PATTERN.find(&flat_macro_value) {
        // Insert sub-macro
        let sub_macro_name = &flat_macro_value[found.start()+2..found.end()-1];
        if !flat_macros.contains_key(sub_macro_name) {
            flatten_macro(&sub_macro_name, history, macros, flat_macros)?;
        }
        flat_macro_value.replace_range(
            found.start()..found.end(),
            flat_macros.get(sub_macro_name).ok_or(MacroError::NotFound(sub_macro_name.to_owned()))?
        );
    }
    // Register flat macro
    flat_macros.insert(
        macro_name.to_owned(),
        flat_macro_value
    );
    // Everything alright
    Ok(())
}


// Tests
#[cfg(test)]
mod tests {
    use super::{
        super::error::MacroError,
        HashMap,
        HashSet
    };


    #[test]
    fn flatten_macro_success() {
        // Test data
        let mut macros = HashMap::new();
        macros.insert("a".to_owned(), "Hello ${b} test!".to_owned());
        macros.insert("b".to_owned(), "fr${c}".to_owned());
        macros.insert("c".to_owned(), "om".to_owned());
        let mut flat_macros = HashMap::new();
        // Test execution
        super::flatten_macro("a", &mut HashSet::new(), &macros, &mut flat_macros).unwrap();
        assert_eq!(flat_macros.get("a").unwrap(), "Hello from test!");
    }
    #[test]
    fn flatten_macro_infinite() {
        // Test data
        let mut macros = HashMap::new();
        macros.insert("a".to_owned(), "foo ${b}".to_owned());
        macros.insert("b".to_owned(), "${a} bar".to_owned());
        // Test execution
        assert_eq!(super::flatten_macro("a", &mut HashSet::new(), &macros, &mut HashMap::new()).unwrap_err(), MacroError::InfiniteLoop("a".to_owned()));
    }
    #[test]
    fn flatten_macro_notfound() {
        assert_eq!(super::flatten_macro("x", &mut HashSet::new(), &HashMap::new(), &mut HashMap::new()).unwrap_err(), MacroError::NotFound("x".to_owned()));
    }
}