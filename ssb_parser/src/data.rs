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
            if line.ends_with("\r") {line.pop();}
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
                            else if let Some(separator_pos) = line.find(KEY_SUFFIX).filter(|pos| *pos > 0) {
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
                                self.target_width = Some(
                                    line[TARGET_WIDTH_KEY.len()..].parse().map_err(|_| ParseError::new_with_pos("Invalid target width value!", (line_index, TARGET_WIDTH_KEY.len())) )?
                                );
                            }
                            // Height
                            else if line.starts_with(TARGET_HEIGHT_KEY) {
                                self.target_height = Some(
                                    line[TARGET_HEIGHT_KEY.len()..].parse().map_err(|_| ParseError::new_with_pos("Invalid target height value!", (line_index, TARGET_HEIGHT_KEY.len())) )?
                                );
                            }
                            // Depth
                            else if line.starts_with(TARGET_DEPTH_KEY) {
                                self.target_depth = line[TARGET_DEPTH_KEY.len()..].parse().map_err(|_| ParseError::new_with_pos("Invalid target depth value!", (line_index, TARGET_DEPTH_KEY.len())) )?;
                            }
                            // View
                            else if line.starts_with(TARGET_VIEW_KEY) {
                                self.target_view = View::try_from(&line[TARGET_VIEW_KEY.len()..]).map_err(|_| ParseError::new_with_pos("Invalid target view value!", (line_index, TARGET_VIEW_KEY.len())) )?;
                            }
                            // Invalid entry
                            else {
                                return Err(ParseError::new_with_pos("Invalid target entry!", (line_index, 0)));
                            }
                        }
                        // Macros section
                        Some(Section::Macros) => {
                            // Macro
                            if let Some(separator_pos) = line.find(KEY_SUFFIX).filter(|pos| *pos > 0) {
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
                            let mut event_tokens = line.splitn(4, EVENT_SEPARATOR);
                            if let (Some(trigger), Some(macro_name), Some(note), Some(data)) = (event_tokens.next(), event_tokens.next(), event_tokens.next(), event_tokens.next()) {
                                // Save event
                                self.events.push(
                                    Event {
                                        trigger: {
                                            // Tag
                                            if trigger.starts_with("'") && trigger.len() >= 2 && trigger.ends_with("'") {
                                                EventTrigger::Id(trigger[1..trigger.len()-1].to_owned())
                                            // Time
                                            } else if let Some(seperator_pos) = trigger.find(TRIGGER_SEPARATOR) {
                                                EventTrigger::Time((
                                                    parse_timestamp(&trigger[..seperator_pos]).map_err(|_| ParseError::new_with_pos("Start timestamp invalid!", (line_index, 0)) )?,
                                                    parse_timestamp(&trigger[seperator_pos + TRIGGER_SEPARATOR.len()..]).map_err(|_| ParseError::new_with_pos("End timestamp invalid!", (line_index, seperator_pos + TRIGGER_SEPARATOR.len())) )?
                                                ))
                                            // Invalid
                                            } else {
                                                return Err(ParseError::new_with_pos("Invalid trigger format!", (line_index, 0)));
                                            }
                                        },
                                        macro_name: Some(macro_name.to_owned()).filter(|s| !s.is_empty()),
                                        note: Some(note.to_owned()).filter(|s| !s.is_empty()),
                                        data: data.to_owned(),
                                        data_location: (line_index, trigger.len() + macro_name.len() + note.len() + EVENT_SEPARATOR.len() * 3)
                                    }
                                );
                            }
                            // Invalid entry
                            else {
                                return Err(ParseError::new_with_pos("Invalid events entry!", (line_index, 0)));
                            }
                        }
                        // Resources section
                        Some(Section::Resources) => {
                            // Font
                            if line.starts_with(RESOURCES_FONT_KEY) {
                                // Parse tokens
                                let mut font_tokens = line[RESOURCES_FONT_KEY.len()..].splitn(3, VALUE_SEPARATOR);
                                if let (Some(family), Some(style), Some(data)) = (font_tokens.next(), font_tokens.next(), font_tokens.next()) {
                                    // Save font
                                    self.fonts.insert(
                                        FontFace {
                                            family: family.to_owned(),
                                            style: FontStyle::try_from(style).map_err(|_| ParseError::new_with_pos("Font style invalid!", (line_index, RESOURCES_FONT_KEY.len() + family.len() + VALUE_SEPARATOR.len())) )?
                                        },
                                        base64::decode(data).map_err(|_| ParseError::new_with_pos("Font data not in base64 format!", (line_index, RESOURCES_FONT_KEY.len() + family.len() + style.len() + (VALUE_SEPARATOR.len() << 1))) )?
                                    );
                                } else {
                                    return Err(ParseError::new_with_pos("Font family, style and data expected!", (line_index, RESOURCES_FONT_KEY.len())));
                                }
                            }
                            // Texture
                            else if line.starts_with(RESOURCES_TEXTURE_KEY) {
                                // Parse tokens
                                let mut texture_tokens = line[RESOURCES_TEXTURE_KEY.len()..].splitn(3, VALUE_SEPARATOR);
                                if let (Some(id), Some(data_type), Some(data)) = (texture_tokens.next(), texture_tokens.next(), texture_tokens.next()) {
                                    // Save texture
                                    self.textures.insert(
                                        id.to_owned(),
                                        match TextureDataType::try_from(data_type).map_err(|_| ParseError::new_with_pos("Texture data type invalid!", (line_index, RESOURCES_TEXTURE_KEY.len() + id.len() + VALUE_SEPARATOR.len())) )? {
                                            // Raw data
                                            TextureDataType::Raw => {
                                                base64::decode(data).map_err(|_| ParseError::new_with_pos("Texture data not in base64 format!", (line_index, RESOURCES_TEXTURE_KEY.len() + id.len() + data_type.len() + (VALUE_SEPARATOR.len() << 1))) )?
                                            }
                                            // Data by url
                                            TextureDataType::Url => {
                                                let full_path = search_path.unwrap_or(Path::new(".")).join(data);
                                                std::fs::read(&full_path).map_err(|err| {
                                                    ParseError::new_with_source(
                                                        &format!("Texture data not loadable from file '{}'!", full_path.display()),
                                                        (line_index, RESOURCES_TEXTURE_KEY.len() + id.len() + data_type.len() + (VALUE_SEPARATOR.len() << 1)),
                                                        err
                                                    )
                                                })?
                                            }
                                        }
                                    );
                                } else {
                                    return Err(ParseError::new_with_pos("Texture id, data type and data expected!", (line_index, RESOURCES_TEXTURE_KEY.len())));
                                }
                            }
                            // Invalid entry
                            else {
                                return Err(ParseError::new_with_pos("Invalid resources entry!", (line_index, 0)));
                            }
                        }
                        // Unset section
                        None => return Err(ParseError::new_with_pos("No section set!", (line_index, 0)))
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
            flatten_macro(macro_name, &mut HashSet::new(), &data.macros, &mut flat_macros).map_err(|err| ParseError::new(&format!("Flattening macro '{}' caused error: {:?}", macro_name, err)) )?;
        }
        // Evaluate events
        let mut events = Vec::with_capacity(data.events.len());
        for event in data.events {
            // Insert base macro
            let mut event_data = event.data.clone();
            if let Some(macro_name) = &event.macro_name {
                event_data.insert_str(0, flat_macros.get(macro_name).ok_or_else(|| ParseError::new(&format!("Base macro '{}' not found to insert in event at line {}", macro_name, event.data_location.0)) )?);
            }
            // Insert inline macros
            while let Some(found) = MACRO_PATTERN.find(&event_data) {
                let macro_name = &event_data[found.start()+2..found.end()-1];
                event_data.replace_range(
                    found.start()..found.end(),
                    flat_macros.get(macro_name).ok_or_else(|| ParseError::new(&format!("Inline macro '{}' not found to insert in event at line {}", macro_name, event.data_location.0)) )?
                );
            }
            // Collect event objects by line tokens
            let mut objects = vec!();
            let mode = Mode::default();
            for (is_tag, data) in TagGeometryIterator::new(&event_data) {
                // Tags
                if is_tag {

                    // TODO

                // Geometries
                } else {
                    match mode {
                        Mode::Text => objects.push(EventObject::Geometry(EventGeometry::Text(data))),
                        Mode::Points => {

                            // TODO

                        },
                        Mode::Shape => {

                            // TODO

                        }
                    }
                }
            }
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
const VALUE_SEPARATOR: &str = ",";
const EVENT_SEPARATOR: &str = "|";
const TRIGGER_SEPARATOR: &str = "-";
const TAG_START: &str = "[";
const TAG_START_CHAR: char = '[';
const TAG_END: &str = "]";
const TAG_END_CHAR: char = ']';
lazy_static! {
    static ref MACRO_PATTERN: Regex = Regex::new("\\$\\{([a-zA-Z0-9_-]+)\\}").unwrap();
    static ref TIMESTAMP_PATTERN: Regex = Regex::new("^(?:(?:(?P<H>\\d{0,2}):(?P<HM>[0-5]?\\d?):)|(?:(?P<M>[0-5]?\\d?):))?(?:(?P<S>[0-5]?\\d?)\\.)?(?P<MS>\\d{0,3})$").unwrap();
}

fn parse_timestamp(timestamp: &str) -> Result<u32,()> {
    // Milliseconds factors
    const MS_2_MS: u32 = 1;
    const S_2_MS: u32 = MS_2_MS * 1000;
    const M_2_MS: u32 = S_2_MS * 60;
    const H_2_MS: u32 = M_2_MS * 60;
    // Calculate time in milliseconds
    let mut ms = 0u32;
    let captures = TIMESTAMP_PATTERN.captures(timestamp).ok_or_else(|| ())?;
    for (unit, factor) in &[("MS", MS_2_MS), ("S", S_2_MS), ("M", M_2_MS), ("HM", M_2_MS), ("H", H_2_MS)] {
        if let Some(unit_value) = captures.name(unit) {
            if unit_value.start() != unit_value.end() { // Not empty
                ms += unit_value.as_str().parse::<u32>().map_err(|_| ())? * factor;
            }
        }
    }
    // Return time
    Ok(ms)
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

struct TagGeometryIterator {
    text: String,
    pos: usize
}
impl TagGeometryIterator {
    pub fn new(source: &str) -> Self {
        Self {
            text: source.replace("\\\\", "\x1B").replace(&("\\".to_owned() + TAG_START), "\x02").replace(&("\\".to_owned() + TAG_END), "\x03").replace("\\n", "\n").replace("\x1B", "\\"),
            pos: 0
        }
    }
}
impl Iterator for TagGeometryIterator {
    type Item = (bool, String);
    fn next(&mut self) -> Option<Self::Item> {
        // Remaining of source
        let text = &self.text[self.pos..];
        // End of source reached?
        if text.is_empty() {
            return None;
        }
        // Match tag or geometry
        let is_tag;
        let text_chunk;
        if text.starts_with(TAG_START) {
            is_tag = true;
            // Till tag end (considers nested tags)
            let mut tag_open_count = 0usize;
            if let Some(end_pos) = text.char_indices().skip(1).find(|c| match c.1 {
                TAG_START_CHAR => {tag_open_count+=1; false},
                TAG_END_CHAR => if tag_open_count == 0 {true} else {tag_open_count-=1; false}
                _ => false
            }).map(|c| c.0) {
                self.pos += end_pos + TAG_END.len();
                text_chunk = &text[TAG_START.len()..end_pos];
            // Till end
            } else {
                self.pos += text.len();
                text_chunk = &text[TAG_START.len()..];
            }
        } else {
            is_tag = false;
            // Till tag start
            if let Some(start_pos) = text.find(TAG_START) {
                self.pos += start_pos;
                text_chunk = &text[..start_pos];
            // Till end
            } else {
                self.pos += text.len();
                text_chunk = text;
            }
        }
        // Return tag or geometry with unescaped characters
        Some((is_tag, text_chunk.replace("\x02", TAG_START).replace("\x03", TAG_END)))
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::{
        parse_timestamp,
        flatten_macro,
        TagGeometryIterator,
        super::error::MacroError,
        HashMap,
        HashSet
    };

    #[test]
    fn parse_timestamp_various() {
        assert_eq!(parse_timestamp(""), Ok(0));
        assert_eq!(parse_timestamp("1:2.3"), Ok(62_003));
        assert_eq!(parse_timestamp("59:59.999"), Ok(3_599_999));
        assert_eq!(parse_timestamp("1::.1"), Ok(3_600_001));
    }

    #[test]
    fn flatten_macro_success() {
        // Test data
        let mut macros = HashMap::new();
        macros.insert("a".to_owned(), "Hello ${b} test!".to_owned());
        macros.insert("b".to_owned(), "fr${c}".to_owned());
        macros.insert("c".to_owned(), "om".to_owned());
        let mut flat_macros = HashMap::new();
        // Test execution
        flatten_macro("a", &mut HashSet::new(), &macros, &mut flat_macros).unwrap();
        assert_eq!(flat_macros.get("a").unwrap(), "Hello from test!");
    }
    #[test]
    fn flatten_macro_infinite() {
        // Test data
        let mut macros = HashMap::new();
        macros.insert("a".to_owned(), "foo ${b}".to_owned());
        macros.insert("b".to_owned(), "${a} bar".to_owned());
        // Test execution
        assert_eq!(flatten_macro("a", &mut HashSet::new(), &macros, &mut HashMap::new()).unwrap_err(), MacroError::InfiniteLoop("a".to_owned()));
    }
    #[test]
    fn flatten_macro_notfound() {
        assert_eq!(flatten_macro("x", &mut HashSet::new(), &HashMap::new(), &mut HashMap::new()).unwrap_err(), MacroError::NotFound("x".to_owned()));
    }

    #[test]
    fn tag_geometry_iter() {
        let mut iter = TagGeometryIterator::new("[tag1][tag2=[inner_tag]]geometry1\\[geometry1_continue\\\\[tag3]geometry2\\n[tag4");
        assert_eq!(iter.next(), Some((true, "tag1".to_owned())));
        assert_eq!(iter.next(), Some((true, "tag2=[inner_tag]".to_owned())));
        assert_eq!(iter.next(), Some((false, "geometry1[geometry1_continue\\".to_owned())));
        assert_eq!(iter.next(), Some((true, "tag3".to_owned())));
        assert_eq!(iter.next(), Some((false, "geometry2\n".to_owned())));
        assert_eq!(iter.next(), Some((true, "tag4".to_owned())));
        assert_eq!(iter.next(), None);
    }
}