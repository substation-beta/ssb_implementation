// Imports
use super::{
    types::{
        error::ParseError,
        state::{Section,Mode,ShapeSegmentType,TextureDataType},
        ssb::{View,Event,EventRender,EventTrigger,EventObject,Point2D,Point3D,FontFace,FontStyle,FontData,TextureId,TextureData},
        geometries::{EventGeometry,ShapeSegment},
        tags::{EventTag,Alignment,Numpad,Margin,WrapStyle,Direction,Space,Rotate,Scale,Translate,Shear,Border,Join,Cap,TexFill,TextureWrapping,Color,Alpha,Blur,Blend,Target,MaskMode}
    },
    utils::{
        constants::*,
        functions::{parse_timestamp,flatten_macro,EscapedText,TagsIterator,bool_from_str,alpha_from_str,rgb_from_str,map_or_err_str,map_else_err_str}
    }
};
use std::{
    collections::{HashMap,HashSet},
    io::BufRead,
    path::Path,
    convert::TryFrom
};


// Raw data
#[derive(Debug, PartialEq)]
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
    pub fn parse_owned<R>(mut self, reader: R, search_path: Option<&Path>) -> Result<Self, ParseError>
        where R: BufRead {
        self.parse(reader, search_path)?;
        Ok(self)
    }
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
                                                    parse_timestamp(&trigger[seperator_pos + 1 /* TRIGGER_SEPARATOR */..]).map_err(|_| ParseError::new_with_pos("End timestamp invalid!", (line_index, seperator_pos + 1 /* TRIGGER_SEPARATOR */) ))?
                                                ))
                                            // Invalid
                                            } else {
                                                return Err(ParseError::new_with_pos("Invalid trigger format!", (line_index, 0)));
                                            }
                                        },
                                        macro_name: Some(macro_name.to_owned()).filter(|s| !s.is_empty()),
                                        note: Some(note.to_owned()).filter(|s| !s.is_empty()),
                                        data: data.to_owned(),
                                        data_location: (line_index, trigger.len() + macro_name.len() + note.len() + 3 /* 3x EVENT_SEPARATOR */)
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
                                            style: FontStyle::try_from(style).map_err(|_| ParseError::new_with_pos("Font style invalid!", (line_index, RESOURCES_FONT_KEY.len() + family.len() + 1 /* VALUE_SEPARATOR */) ))?
                                        },
                                        base64::decode(data).map_err(|_| ParseError::new_with_pos("Font data not in base64 format!", (line_index, RESOURCES_FONT_KEY.len() + family.len() + style.len() + (1 /* VALUE_SEPARATOR */ << 1))) )?
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
                                        match TextureDataType::try_from(data_type).map_err(|_| ParseError::new_with_pos("Texture data type invalid!", (line_index, RESOURCES_TEXTURE_KEY.len() + id.len() + 1 /* VALUE_SEPARATOR */)) )? {
                                            // Raw data
                                            TextureDataType::Raw => {
                                                base64::decode(data).map_err(|_| ParseError::new_with_pos("Texture data not in base64 format!", (line_index, RESOURCES_TEXTURE_KEY.len() + id.len() + data_type.len() + (1 /* VALUE_SEPARATOR */ << 1))) )?
                                            }
                                            // Data by url
                                            TextureDataType::Url => {
                                                let full_path = search_path.unwrap_or_else(|| Path::new(".")).join(data);
                                                std::fs::read(&full_path).map_err(|err| {
                                                    ParseError::new_with_source(
                                                        &format!("Texture data not loadable from file '{}'!", full_path.display()),
                                                        (line_index, RESOURCES_TEXTURE_KEY.len() + id.len() + data_type.len() + (1 /* VALUE_SEPARATOR */ << 1)),
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
#[derive(Debug, PartialEq)]
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
                event_data.insert_str(0, flat_macros.get(macro_name.as_str()).ok_or_else(|| ParseError::new_with_pos(&format!("Base macro '{}' not found to insert!", macro_name), (event.data_location.0, 0)) )?);
            }
            // Insert inline macros
            while let Some(found) = MACRO_PATTERN.find(&event_data) {
                let macro_name = &event_data[found.start()+MACRO_INLINE_START.len()..found.end()-MACRO_INLINE_END.len()];
                let macro_location = found.start()..found.end();
                let macro_value = flat_macros.get(macro_name).ok_or_else(|| ParseError::new_with_pos(&format!("Inline macro '{}' not found to insert!", macro_name), event.data_location) )?;
                event_data.replace_range(macro_location, macro_value);
            }
            // Collect event objects by line tokens
            let mut objects = vec![];
            let mut mode = Mode::default();
            for (is_tag, data) in EscapedText::new(&event_data).iter() {
                // Tags
                if is_tag {
                    for (tag_name, tag_value) in TagsIterator::new(data) {
                        match tag_name {
                            "font" => objects.push(EventObject::Tag(EventTag::Font(
                                map_else_err_str(tag_value, |value| Some(value.to_owned()) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid font '{}'!", value), event.data_location) )?
                            ))),
                            "size" => objects.push(EventObject::Tag(EventTag::Size(
                                map_or_err_str(tag_value, |value| value.parse() )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid size '{}'!", value), event.data_location) )?
                            ))),
                            "bold" => objects.push(EventObject::Tag(EventTag::Bold(
                                map_or_err_str(tag_value, |value| bool_from_str(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid bold '{}'!", value), event.data_location) )?
                            ))),
                            "italic" => objects.push(EventObject::Tag(EventTag::Italic(
                                map_or_err_str(tag_value, |value| bool_from_str(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid italic '{}'!", value), event.data_location) )?
                            ))),
                            "underline" => objects.push(EventObject::Tag(EventTag::Underline(
                                map_or_err_str(tag_value, |value| bool_from_str(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid underline '{}'!", value), event.data_location) )?
                            ))),
                            "strikeout" => objects.push(EventObject::Tag(EventTag::Strikeout(
                                map_or_err_str(tag_value, |value| bool_from_str(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid strikeout '{}'!", value), event.data_location) )?
                            ))),
                            "position" => objects.push(EventObject::Tag(EventTag::Position(
                                map_else_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                                    Some(Point3D {
                                        x: tokens.next()?.parse().ok()?,
                                        y: tokens.next()?.parse().ok()?,
                                        z: tokens.next().or(Some("0")).and_then(|value| value.parse().ok())?
                                    })
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid position '{}'!", value), event.data_location) )?
                            ))),
                            "alignment" => objects.push(EventObject::Tag(EventTag::Alignment(
                                map_else_err_str(tag_value, |value| {
                                    Some(
                                        if let Some(sep) = value.find(VALUE_SEPARATOR) {
                                            Alignment::Offset(Point2D {
                                                x: value[..sep].parse().ok()?,
                                                y: value[sep + 1 /* VALUE_SEPARATOR */..].parse().ok()?,
                                            })
                                        } else {
                                            Alignment::Numpad(Numpad::try_from(value.parse::<u8>().ok()?).ok()?)
                                        }
                                    )
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid alignment '{}'!", value), event.data_location) )?
                            ))),
                            "margin" => objects.push(EventObject::Tag(EventTag::Margin(
                                map_else_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(4, VALUE_SEPARATOR);
                                    Some(
                                        if let (Some(top), Some(right), Some(bottom), Some(left)) = (tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                                            Margin::All(
                                                top.parse().ok()?,
                                                right.parse().ok()?,
                                                bottom.parse().ok()?,
                                                left.parse().ok()?
                                            )
                                        } else {
                                            let margin = value.parse().ok()?;
                                            Margin::All(
                                                margin,
                                                margin,
                                                margin,
                                                margin
                                            )
                                        }
                                    )
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid margin '{}'!", value), event.data_location) )?
                            ))),
                            "margin-top" => objects.push(EventObject::Tag(EventTag::Margin(
                                map_else_err_str(tag_value, |value| Some(Margin::Top(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid margin top '{}'!", value), event.data_location) )?
                            ))),
                            "margin-right" => objects.push(EventObject::Tag(EventTag::Margin(
                                map_else_err_str(tag_value, |value| Some(Margin::Right(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid margin right '{}'!", value), event.data_location) )?
                            ))),
                            "margin-bottom" => objects.push(EventObject::Tag(EventTag::Margin(
                                map_else_err_str(tag_value, |value| Some(Margin::Bottom(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid margin bottom '{}'!", value), event.data_location) )?
                            ))),
                            "margin-left" => objects.push(EventObject::Tag(EventTag::Margin(
                                map_else_err_str(tag_value, |value| Some(Margin::Left(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid margin left '{}'!", value), event.data_location) )?
                            ))),
                            "wrap-style" => objects.push(EventObject::Tag(EventTag::WrapStyle(
                                map_or_err_str(tag_value, |value| WrapStyle::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid wrap style '{}'!", value), event.data_location) )?
                            ))),
                            "direction" => objects.push(EventObject::Tag(EventTag::Direction(
                                map_or_err_str(tag_value, |value| Direction::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid direction '{}'!", value), event.data_location) )?
                            ))),
                            "space" => objects.push(EventObject::Tag(EventTag::Space(
                                map_else_err_str(tag_value, |value| {
                                    Some(
                                        if let Some(sep) = value.find(VALUE_SEPARATOR) {
                                            Space::All(
                                                value[..sep].parse().ok()?,
                                                value[sep + 1 /* VALUE_SEPARATOR */..].parse().ok()?
                                            )
                                        } else {
                                            let space = value.parse().ok()?;
                                            Space::All(space, space)
                                        }
                                    )
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid space '{}'!", value), event.data_location) )?
                            ))),
                            "space-h" => objects.push(EventObject::Tag(EventTag::Space(
                                map_else_err_str(tag_value, |value| Some(Space::Horizontal(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid space horizontal '{}'!", value), event.data_location) )?
                            ))),
                            "space-v" => objects.push(EventObject::Tag(EventTag::Space(
                                map_else_err_str(tag_value, |value| Some(Space::Vertical(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid space vertical '{}'!", value), event.data_location) )?
                            ))),
                            "rotate" => objects.push(EventObject::Tag(EventTag::Rotate(
                                map_else_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                                    Some(Rotate::All(
                                        tokens.next()?.parse().ok()?,
                                        tokens.next()?.parse().ok()?,
                                        tokens.next()?.parse().ok()?
                                    ))
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid rotate '{}'!", value), event.data_location) )?
                            ))),
                            "rotate-x" => objects.push(EventObject::Tag(EventTag::Rotate(
                                map_else_err_str(tag_value, |value| Some(Rotate::X(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid rotate x '{}'!", value), event.data_location) )?
                            ))),
                            "rotate-y" => objects.push(EventObject::Tag(EventTag::Rotate(
                                map_else_err_str(tag_value, |value| Some(Rotate::Y(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid rotate y '{}'!", value), event.data_location) )?
                            ))),
                            "rotate-z" => objects.push(EventObject::Tag(EventTag::Rotate(
                                map_else_err_str(tag_value, |value| Some(Rotate::Z(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid rotate z '{}'!", value), event.data_location) )?
                            ))),
                            "scale" => objects.push(EventObject::Tag(EventTag::Scale(
                                map_else_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                                    Some(Scale::All(
                                        tokens.next()?.parse().ok()?,
                                        tokens.next()?.parse().ok()?,
                                        tokens.next()?.parse().ok()?
                                    ))
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid scale '{}'!", value), event.data_location) )?
                            ))),
                            "scale-x" => objects.push(EventObject::Tag(EventTag::Scale(
                                map_else_err_str(tag_value, |value| Some(Scale::X(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid scale x '{}'!", value), event.data_location) )?
                            ))),
                            "scale-y" => objects.push(EventObject::Tag(EventTag::Scale(
                                map_else_err_str(tag_value, |value| Some(Scale::Y(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid scale y '{}'!", value), event.data_location) )?
                            ))),
                            "scale-z" => objects.push(EventObject::Tag(EventTag::Scale(
                                map_else_err_str(tag_value, |value| Some(Scale::Z(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid scale z '{}'!", value), event.data_location) )?
                            ))),
                            "translate" => objects.push(EventObject::Tag(EventTag::Translate(
                                map_else_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                                    Some(Translate::All(
                                        tokens.next()?.parse().ok()?,
                                        tokens.next()?.parse().ok()?,
                                        tokens.next()?.parse().ok()?
                                    ))
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid translate '{}'!", value), event.data_location) )?
                            ))),
                            "translate-x" => objects.push(EventObject::Tag(EventTag::Translate(
                                map_else_err_str(tag_value, |value| Some(Translate::X(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid translate x '{}'!", value), event.data_location) )?
                            ))),
                            "translate-y" => objects.push(EventObject::Tag(EventTag::Translate(
                                map_else_err_str(tag_value, |value| Some(Translate::Y(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid translate y '{}'!", value), event.data_location) )?
                            ))),
                            "translate-z" => objects.push(EventObject::Tag(EventTag::Translate(
                                map_else_err_str(tag_value, |value| Some(Translate::Z(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid translate z '{}'!", value), event.data_location) )?
                            ))),
                            "shear" => objects.push(EventObject::Tag(EventTag::Shear(
                                map_else_err_str(tag_value, |value| {
                                    let sep = value.find(VALUE_SEPARATOR)?;
                                    Some(Shear::All(
                                        value[..sep].parse().ok()?,
                                        value[sep + 1 /* VALUE_SEPARATOR */..].parse().ok()?
                                    ))
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid shear '{}'!", value), event.data_location) )?
                            ))),
                            "shear-x" => objects.push(EventObject::Tag(EventTag::Shear(
                                map_else_err_str(tag_value, |value| Some(Shear::X(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid shear x '{}'!", value), event.data_location) )?
                            ))),
                            "shear-y" => objects.push(EventObject::Tag(EventTag::Shear(
                                map_else_err_str(tag_value, |value| Some(Shear::Y(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid shear y '{}'!", value), event.data_location) )?
                            ))),
                            "matrix" => objects.push(EventObject::Tag(EventTag::Matrix(
                                map_else_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(16, VALUE_SEPARATOR).filter_map(|value| value.parse().ok() );
                                    Some(Box::new([
                                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?,
                                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?,
                                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?,
                                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?
                                    ]))
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid matrix '{}'!", value), event.data_location) )?
                            ))),
                            "mode" => mode =
                                map_or_err_str(tag_value, |value| Mode::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid mode '{}'!", value), event.data_location) )?,
                            "border" => objects.push(EventObject::Tag(EventTag::Border(
                                map_else_err_str(tag_value, |value| {
                                    Some(
                                        if let Some(sep) = value.find(VALUE_SEPARATOR) {
                                            Border::All(
                                                value[..sep].parse().ok()?,
                                                value[sep + 1 /* VALUE_SEPARATOR */..].parse().ok()?
                                            )
                                        } else {
                                            let border = value.parse().ok()?;
                                            Border::All(border, border)
                                        }
                                    )
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid border '{}'!", value), event.data_location) )?
                            ))),
                            "border-h" => objects.push(EventObject::Tag(EventTag::Border(
                                map_else_err_str(tag_value, |value| Some(Border::Horizontal(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid border horizontal '{}'!", value), event.data_location) )?
                            ))),
                            "border-v" => objects.push(EventObject::Tag(EventTag::Border(
                                map_else_err_str(tag_value, |value| Some(Border::Vertical(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid border vertical '{}'!", value), event.data_location) )?
                            ))),
                            "join" => objects.push(EventObject::Tag(EventTag::Join(
                                map_or_err_str(tag_value, |value| Join::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid join '{}'!", value), event.data_location) )?
                            ))),
                            "cap" => objects.push(EventObject::Tag(EventTag::Cap(
                                map_or_err_str(tag_value, |value| Cap::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid cap '{}'!", value), event.data_location) )?
                            ))),
                            "texture" => objects.push(EventObject::Tag(EventTag::Texture(
                                map_else_err_str(tag_value, |value| Some(value.to_owned()) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid texture '{}'!", value), event.data_location) )?
                            ))),
                            "texfill" => objects.push(EventObject::Tag(EventTag::TexFill(
                                map_else_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(5, VALUE_SEPARATOR);
                                    Some(TexFill {
                                        x0: tokens.next()?.parse().ok()?,
                                        y0: tokens.next()?.parse().ok()?,
                                        x1: tokens.next()?.parse().ok()?,
                                        y1: tokens.next()?.parse().ok()?,
                                        wrap: TextureWrapping::try_from(tokens.next()?).ok()?
                                    })
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid texture filling '{}'!", value), event.data_location) )?
                            ))),
                            "color" => objects.push(EventObject::Tag(EventTag::Color(
                                map_or_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(5, VALUE_SEPARATOR);
                                    Ok(match (tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                                        (Some(color1), Some(color2), Some(color3), Some(color4), Some(color5)) =>
                                            Color::CornersWithStop([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?,
                                                rgb_from_str(color3)?,
                                                rgb_from_str(color4)?,
                                                rgb_from_str(color5)?
                                            ]),
                                        (Some(color1), Some(color2), Some(color3), Some(color4), None) =>
                                            Color::Corners([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?,
                                                rgb_from_str(color3)?,
                                                rgb_from_str(color4)?
                                            ]),
                                        (Some(color1), Some(color2), Some(color3), None, None) =>
                                            Color::LinearWithStop([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?,
                                                rgb_from_str(color3)?
                                            ]),
                                        (Some(color1), Some(color2), None, None, None) =>
                                            Color::Linear([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?
                                            ]),
                                        (Some(color1), None, None, None, None) =>
                                            Color::Mono(
                                                rgb_from_str(color1)?
                                            ),
                                        _ => return Err(())
                                    })
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid color '{}'!", value), event.data_location) )?
                            ))),
                            "bordercolor" => objects.push(EventObject::Tag(EventTag::BorderColor(
                                map_or_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(5, VALUE_SEPARATOR);
                                    Ok(match (tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                                        (Some(color1), Some(color2), Some(color3), Some(color4), Some(color5)) =>
                                            Color::CornersWithStop([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?,
                                                rgb_from_str(color3)?,
                                                rgb_from_str(color4)?,
                                                rgb_from_str(color5)?
                                            ]),
                                        (Some(color1), Some(color2), Some(color3), Some(color4), None) =>
                                            Color::Corners([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?,
                                                rgb_from_str(color3)?,
                                                rgb_from_str(color4)?
                                            ]),
                                        (Some(color1), Some(color2), Some(color3), None, None) =>
                                            Color::LinearWithStop([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?,
                                                rgb_from_str(color3)?
                                            ]),
                                        (Some(color1), Some(color2), None, None, None) =>
                                            Color::Linear([
                                                rgb_from_str(color1)?,
                                                rgb_from_str(color2)?
                                            ]),
                                        (Some(color1), None, None, None, None) =>
                                            Color::Mono(
                                                rgb_from_str(color1)?
                                            ),
                                        _ => return Err(())
                                    })
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid color '{}'!", value), event.data_location) )?
                            ))),
                            "alpha" => objects.push(EventObject::Tag(EventTag::Alpha(
                                map_or_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(5, VALUE_SEPARATOR);
                                    Ok(match (tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                                        (Some(alpha1), Some(alpha2), Some(alpha3), Some(alpha4), Some(alpha5)) =>
                                            Alpha::CornersWithStop([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?,
                                                alpha_from_str(alpha3)?,
                                                alpha_from_str(alpha4)?,
                                                alpha_from_str(alpha5)?
                                            ]),
                                        (Some(alpha1), Some(alpha2), Some(alpha3), Some(alpha4), None) =>
                                            Alpha::Corners([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?,
                                                alpha_from_str(alpha3)?,
                                                alpha_from_str(alpha4)?
                                            ]),
                                        (Some(alpha1), Some(alpha2), Some(alpha3), None, None) =>
                                            Alpha::LinearWithStop([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?,
                                                alpha_from_str(alpha3)?
                                            ]),
                                        (Some(alpha1), Some(alpha2), None, None, None) =>
                                            Alpha::Linear([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?
                                            ]),
                                        (Some(alpha1), None, None, None, None) =>
                                            Alpha::Mono(
                                                alpha_from_str(alpha1)?
                                            ),
                                        _ => return Err(())
                                    })
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid color '{}'!", value), event.data_location) )?
                            ))),
                            "borderalpha" => objects.push(EventObject::Tag(EventTag::BorderAlpha(
                                map_or_err_str(tag_value, |value| {
                                    let mut tokens = value.splitn(5, VALUE_SEPARATOR);
                                    Ok(match (tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                                        (Some(alpha1), Some(alpha2), Some(alpha3), Some(alpha4), Some(alpha5)) =>
                                            Alpha::CornersWithStop([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?,
                                                alpha_from_str(alpha3)?,
                                                alpha_from_str(alpha4)?,
                                                alpha_from_str(alpha5)?
                                            ]),
                                        (Some(alpha1), Some(alpha2), Some(alpha3), Some(alpha4), None) =>
                                            Alpha::Corners([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?,
                                                alpha_from_str(alpha3)?,
                                                alpha_from_str(alpha4)?
                                            ]),
                                        (Some(alpha1), Some(alpha2), Some(alpha3), None, None) =>
                                            Alpha::LinearWithStop([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?,
                                                alpha_from_str(alpha3)?
                                            ]),
                                        (Some(alpha1), Some(alpha2), None, None, None) =>
                                            Alpha::Linear([
                                                alpha_from_str(alpha1)?,
                                                alpha_from_str(alpha2)?
                                            ]),
                                        (Some(alpha1), None, None, None, None) =>
                                            Alpha::Mono(
                                                alpha_from_str(alpha1)?
                                            ),
                                        _ => return Err(())
                                    })
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid color '{}'!", value), event.data_location) )?
                            ))),
                            "blur" => objects.push(EventObject::Tag(EventTag::Blur(
                                map_else_err_str(tag_value, |value| {
                                    Some(
                                        if let Some(sep) = value.find(VALUE_SEPARATOR) {
                                            Blur::All(
                                                value[..sep].parse().ok()?,
                                                value[sep + 1 /* VALUE_SEPARATOR */..].parse().ok()?
                                            )
                                        } else {
                                            let blur = value.parse().ok()?;
                                            Blur::All(blur, blur)
                                        }
                                    )
                                } )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid blur '{}'!", value), event.data_location) )?
                            ))),
                            "blur-h" => objects.push(EventObject::Tag(EventTag::Blur(
                                map_else_err_str(tag_value, |value| Some(Blur::Horizontal(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid blur horizontal '{}'!", value), event.data_location) )?
                            ))),
                            "blur-v" => objects.push(EventObject::Tag(EventTag::Blur(
                                map_else_err_str(tag_value, |value| Some(Blur::Vertical(value.parse().ok()?)) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid blur vertical '{}'!", value), event.data_location) )?
                            ))),
                            "blend" => objects.push(EventObject::Tag(EventTag::Blend(
                                map_or_err_str(tag_value, |value| Blend::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid blend '{}'!", value), event.data_location) )?
                            ))),
                            "target" => objects.push(EventObject::Tag(EventTag::Target(
                                map_or_err_str(tag_value, |value| Target::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid target '{}'!", value), event.data_location) )?
                            ))),
                            "mask-mode" => objects.push(EventObject::Tag(EventTag::MaskMode(
                                map_or_err_str(tag_value, |value| MaskMode::try_from(value) )
                                .map_err(|value| ParseError::new_with_pos(&format!("Invalid mask mode '{}'!", value), event.data_location) )?
                            ))),
                            "mask-clear" => objects.push(EventObject::Tag(
                                if tag_value.is_none() {EventTag::MaskClear}
                                else {return Err(ParseError::new_with_pos("Mask clear has no value!", event.data_location))}
                            )),
                            

                            _ if !tag_name.is_empty() => println!("{}={:?}", tag_name, tag_value), // TODO: all other tags


                            _ => return Err(ParseError::new_with_pos(&format!("Invalid tag '{}'!", tag_name), event.data_location))
                        }
                    }
                // Geometries
                } else {
                    match mode {
                        Mode::Text => objects.push(EventObject::Geometry(EventGeometry::Text(data.to_owned()))),
                        Mode::Points => {
                            // Find points
                            let tokens = data.split_ascii_whitespace().collect::<Vec<&str>>();
                            let mut points = Vec::with_capacity(tokens.len() >> 1);
                            let mut tokens = tokens.iter();
                            // Collect points
                            loop {
                                match (tokens.next(), tokens.next()) {
                                    (Some(x), Some(y)) => points.push(Point2D {
                                        x: x.parse().map_err(|_| ParseError::new_with_pos(&format!("Invalid X coordinate of point '{}'!", x), event.data_location) )?,
                                        y: y.parse().map_err(|_| ParseError::new_with_pos(&format!("Invalid Y coordinate of point '{}'!", y), event.data_location) )?
                                    }),
                                    (Some(leftover), None) => return Err(ParseError::new_with_pos(&format!("Points incomplete (leftover: '{}')!", leftover), event.data_location)),
                                    _ => break
                                }
                            }
                            // Save points
                            objects.push(EventObject::Geometry(EventGeometry::Points(points)));
                        }
                        Mode::Shape => {
                            // Find segments
                            let tokens = data.split_ascii_whitespace().collect::<Vec<&str>>();
                            let mut segments = Vec::with_capacity(tokens.len() >> 2 /* Vague estimation, shrinking later */);   
                            let mut tokens = tokens.iter();
                            // Collect segments
                            let mut segment_type = ShapeSegmentType::default();
                            while let Some(token) = tokens.next() {
                                match *token {
                                    "m" => segment_type = ShapeSegmentType::Move,
                                    "l" => segment_type = ShapeSegmentType::Line,
                                    "b" => segment_type = ShapeSegmentType::Curve,
                                    "a" => segment_type = ShapeSegmentType::Arc,
                                    "c" => {segments.push(ShapeSegment::Close); segment_type = ShapeSegmentType::Move;}
                                    _ => match segment_type {
                                        ShapeSegmentType::Move => segments.push(ShapeSegment::MoveTo(Point2D {
                                            x: token.parse().map_err(|_| ParseError::new_with_pos(&format!("Invalid X coordinate of move '{}'!", token), event.data_location) )?,
                                            y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid Y coordinate of move '{}'!", token), event.data_location) )?
                                        })),
                                        ShapeSegmentType::Line => segments.push(ShapeSegment::LineTo(Point2D {
                                            x: token.parse().map_err(|_| ParseError::new_with_pos(&format!("Invalid X coordinate of line '{}'!", token), event.data_location) )?,
                                            y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid Y coordinate of line '{}'!", token), event.data_location) )?
                                        })),
                                        ShapeSegmentType::Curve => segments.push(ShapeSegment::CurveTo(
                                            Point2D {
                                                x: token.parse().map_err(|_| ParseError::new_with_pos(&format!("Invalid X coordinate of curve first point '{}'!", token), event.data_location) )?,
                                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid Y coordinate of curve first point '{}'!", token), event.data_location) )?
                                            },
                                            Point2D {
                                                x: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid X coordinate of curve second point '{}'!", token), event.data_location) )?,
                                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid Y coordinate of curve second point '{}'!", token), event.data_location) )?
                                            },
                                            Point2D {
                                                x: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid X coordinate of curve third point '{}'!", token), event.data_location) )?,
                                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid Y coordinate of curve third point '{}'!", token), event.data_location) )?
                                            }
                                        )),
                                        ShapeSegmentType::Arc => segments.push(ShapeSegment::ArcBy(
                                            Point2D {
                                                x: token.parse().map_err(|_| ParseError::new_with_pos(&format!("Invalid X coordinate of arc '{}'!", token), event.data_location) )?,
                                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid Y coordinate of arc '{}'!", token), event.data_location) )?
                                            },
                                            map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new_with_pos(&format!("Invalid degree of arc '{}'!", token), event.data_location) )?
                                        )),
                                    }
                                }
                            }
                            // Save segments
                            segments.shrink_to_fit();
                            objects.push(EventObject::Geometry(EventGeometry::Shape(segments)));
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