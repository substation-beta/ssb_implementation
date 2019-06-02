// Imports
use super::{
    error::*,
    types::*,
    utils::*
};
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
            if line.ends_with('\r') {line.pop();}
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
                                            if trigger.starts_with('\'') && trigger.len() >= 2 && trigger.ends_with('\'') {
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
                                                let full_path = search_path.unwrap_or_else(|| Path::new(".")).join(data);
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
                let macro_name = &event_data[found.start()+MACRO_INLINE_START.len()..found.end()-MACRO_INLINE_END.len()];
                event_data.replace_range(
                    found.start()..found.end(),
                    flat_macros.get(macro_name).ok_or_else(|| ParseError::new(&format!("Inline macro '{}' not found to insert in event at line {}", macro_name, event.data_location.0)) )?
                );
            }
            // Collect event objects by line tokens
            let mut objects = vec!();
            let mode = Mode::default();
            for (is_tag, data) in EscapedText::new(&event_data).iter() {
                // Tags
                if is_tag {

                    // TODO

                // Geometries
                } else {
                    match mode {
                        Mode::Text => objects.push(EventObject::Geometry(EventGeometry::Text(data.to_owned()))),
                        Mode::Points => {

                            // TODO

                        }
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