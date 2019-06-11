// Imports
use super::{
    types::{
        error::ParseError,
        state::{Section,Mode,ShapeSegmentType,TextureDataType},
        ssb::{View,Event,EventRender,EventTrigger,FontFace,FontStyle,FontData,TextureId,TextureData},
        objects::{Point2D,Point3D,EventObject,ShapeSegment,Alignment,Numpad,Margin,WrapStyle,Direction,Space,Rotate,Scale,Translate,Shear,Border,Join,Cap,TexFill,TextureWrapping,Color,Alpha,Blur,Blend,Target,MaskMode,Animate}
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
            // Parse objects and save event for rendering
            events.push(
                EventRender {
                    trigger: event.trigger.clone(),
                    objects: parse_objects(&event_data).map_err(|err| ParseError::new_with_source("Invalid event data!", event.data_location, err) )?
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


// Objects parsing
fn parse_objects(event_data: &str) -> Result<Vec<EventObject>, ParseError> {
    let mut objects = vec![];
    let mut mode = Mode::default();
    for (is_tag, data) in EscapedText::new(event_data).iter() {
        if is_tag {
            parse_tags(data, &mut objects, Some(&mut mode))?;
        } else {
            parse_geometries(data, &mut objects, &mode)?;
        }
    }
    Ok(objects)
}
fn parse_tags(data: &str, objects: &mut Vec<EventObject>, mut mode: Option<&mut Mode>) -> Result<(), ParseError> {
    for (tag_name, tag_value) in TagsIterator::new(data) {
        match tag_name {
            "font" => objects.push(EventObject::TagFont(
                map_else_err_str(tag_value, |value| Some(value.to_owned()) )
                .map_err(|value| ParseError::new(&format!("Invalid font '{}'!", value)) )?
            )),
            "size" => objects.push(EventObject::TagSize(
                map_or_err_str(tag_value, |value| value.parse() )
                .map_err(|value| ParseError::new(&format!("Invalid size '{}'!", value)) )?
            )),
            "bold" => objects.push(EventObject::TagBold(
                map_or_err_str(tag_value, |value| bool_from_str(value) )
                .map_err(|value| ParseError::new(&format!("Invalid bold '{}'!", value)) )?
            )),
            "italic" => objects.push(EventObject::TagItalic(
                map_or_err_str(tag_value, |value| bool_from_str(value) )
                .map_err(|value| ParseError::new(&format!("Invalid italic '{}'!", value)) )?
            )),
            "underline" => objects.push(EventObject::TagUnderline(
                map_or_err_str(tag_value, |value| bool_from_str(value) )
                .map_err(|value| ParseError::new(&format!("Invalid underline '{}'!", value)) )?
            )),
            "strikeout" => objects.push(EventObject::TagStrikeout(
                map_or_err_str(tag_value, |value| bool_from_str(value) )
                .map_err(|value| ParseError::new(&format!("Invalid strikeout '{}'!", value)) )?
            )),
            "position" => objects.push(EventObject::TagPosition(
                map_else_err_str(tag_value, |value| {
                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                    Some(Point3D {
                        x: tokens.next()?.parse().ok()?,
                        y: tokens.next()?.parse().ok()?,
                        z: tokens.next().or(Some("0")).and_then(|value| value.parse().ok())?
                    })
                } )
                .map_err(|value| ParseError::new(&format!("Invalid position '{}'!", value)) )?
            )),
            "alignment" => objects.push(EventObject::TagAlignment(
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
                .map_err(|value| ParseError::new(&format!("Invalid alignment '{}'!", value)) )?
            )),
            "margin" => objects.push(EventObject::TagMargin(
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
                .map_err(|value| ParseError::new(&format!("Invalid margin '{}'!", value)) )?
            )),
            "margin-top" => objects.push(EventObject::TagMargin(
                map_else_err_str(tag_value, |value| Some(Margin::Top(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid margin top '{}'!", value)) )?
            )),
            "margin-right" => objects.push(EventObject::TagMargin(
                map_else_err_str(tag_value, |value| Some(Margin::Right(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid margin right '{}'!", value)) )?
            )),
            "margin-bottom" => objects.push(EventObject::TagMargin(
                map_else_err_str(tag_value, |value| Some(Margin::Bottom(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid margin bottom '{}'!", value)) )?
            )),
            "margin-left" => objects.push(EventObject::TagMargin(
                map_else_err_str(tag_value, |value| Some(Margin::Left(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid margin left '{}'!", value)) )?
            )),
            "wrap-style" => objects.push(EventObject::TagWrapStyle(
                map_or_err_str(tag_value, |value| WrapStyle::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid wrap style '{}'!", value)) )?
            )),
            "direction" => objects.push(EventObject::TagDirection(
                map_or_err_str(tag_value, |value| Direction::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid direction '{}'!", value)) )?
            )),
            "space" => objects.push(EventObject::TagSpace(
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
                .map_err(|value| ParseError::new(&format!("Invalid space '{}'!", value)) )?
            )),
            "space-h" => objects.push(EventObject::TagSpace(
                map_else_err_str(tag_value, |value| Some(Space::Horizontal(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid space horizontal '{}'!", value)) )?
            )),
            "space-v" => objects.push(EventObject::TagSpace(
                map_else_err_str(tag_value, |value| Some(Space::Vertical(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid space vertical '{}'!", value)) )?
            )),
            "rotate" => objects.push(EventObject::TagRotate(
                map_else_err_str(tag_value, |value| {
                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                    Some(Rotate::All(
                        tokens.next()?.parse().ok()?,
                        tokens.next()?.parse().ok()?,
                        tokens.next()?.parse().ok()?
                    ))
                } )
                .map_err(|value| ParseError::new(&format!("Invalid rotate '{}'!", value)) )?
            )),
            "rotate-x" => objects.push(EventObject::TagRotate(
                map_else_err_str(tag_value, |value| Some(Rotate::X(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid rotate x '{}'!", value)) )?
            )),
            "rotate-y" => objects.push(EventObject::TagRotate(
                map_else_err_str(tag_value, |value| Some(Rotate::Y(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid rotate y '{}'!", value)) )?
            )),
            "rotate-z" => objects.push(EventObject::TagRotate(
                map_else_err_str(tag_value, |value| Some(Rotate::Z(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid rotate z '{}'!", value)) )?
            )),
            "scale" => objects.push(EventObject::TagScale(
                map_else_err_str(tag_value, |value| {
                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                    Some(Scale::All(
                        tokens.next()?.parse().ok()?,
                        tokens.next()?.parse().ok()?,
                        tokens.next()?.parse().ok()?
                    ))
                } )
                .map_err(|value| ParseError::new(&format!("Invalid scale '{}'!", value)) )?
            )),
            "scale-x" => objects.push(EventObject::TagScale(
                map_else_err_str(tag_value, |value| Some(Scale::X(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid scale x '{}'!", value)) )?
            )),
            "scale-y" => objects.push(EventObject::TagScale(
                map_else_err_str(tag_value, |value| Some(Scale::Y(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid scale y '{}'!", value)) )?
            )),
            "scale-z" => objects.push(EventObject::TagScale(
                map_else_err_str(tag_value, |value| Some(Scale::Z(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid scale z '{}'!", value)) )?
            )),
            "translate" => objects.push(EventObject::TagTranslate(
                map_else_err_str(tag_value, |value| {
                    let mut tokens = value.splitn(3, VALUE_SEPARATOR);
                    Some(Translate::All(
                        tokens.next()?.parse().ok()?,
                        tokens.next()?.parse().ok()?,
                        tokens.next()?.parse().ok()?
                    ))
                } )
                .map_err(|value| ParseError::new(&format!("Invalid translate '{}'!", value)) )?
            )),
            "translate-x" => objects.push(EventObject::TagTranslate(
                map_else_err_str(tag_value, |value| Some(Translate::X(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid translate x '{}'!", value)) )?
            )),
            "translate-y" => objects.push(EventObject::TagTranslate(
                map_else_err_str(tag_value, |value| Some(Translate::Y(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid translate y '{}'!", value)) )?
            )),
            "translate-z" => objects.push(EventObject::TagTranslate(
                map_else_err_str(tag_value, |value| Some(Translate::Z(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid translate z '{}'!", value)) )?
            )),
            "shear" => objects.push(EventObject::TagShear(
                map_else_err_str(tag_value, |value| {
                    let sep = value.find(VALUE_SEPARATOR)?;
                    Some(Shear::All(
                        value[..sep].parse().ok()?,
                        value[sep + 1 /* VALUE_SEPARATOR */..].parse().ok()?
                    ))
                } )
                .map_err(|value| ParseError::new(&format!("Invalid shear '{}'!", value)) )?
            )),
            "shear-x" => objects.push(EventObject::TagShear(
                map_else_err_str(tag_value, |value| Some(Shear::X(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid shear x '{}'!", value)) )?
            )),
            "shear-y" => objects.push(EventObject::TagShear(
                map_else_err_str(tag_value, |value| Some(Shear::Y(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid shear y '{}'!", value)) )?
            )),
            "matrix" => objects.push(EventObject::TagMatrix(
                map_else_err_str(tag_value, |value| {
                    let mut tokens = value.splitn(16, VALUE_SEPARATOR).filter_map(|value| value.parse().ok() );
                    Some(Box::new([
                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?,
                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?,
                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?,
                        tokens.next()?, tokens.next()?, tokens.next()?, tokens.next()?
                    ]))
                } )
                .map_err(|value| ParseError::new(&format!("Invalid matrix '{}'!", value)) )?
            )),
            "mode" if mode.is_some() => **mode.as_mut().expect("Impossible :O Checked right before!") =
                map_or_err_str(tag_value, |value| Mode::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid mode '{}'!", value)) )?,
            "border" => objects.push(EventObject::TagBorder(
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
                .map_err(|value| ParseError::new(&format!("Invalid border '{}'!", value)) )?
            )),
            "border-h" => objects.push(EventObject::TagBorder(
                map_else_err_str(tag_value, |value| Some(Border::Horizontal(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid border horizontal '{}'!", value)) )?
            )),
            "border-v" => objects.push(EventObject::TagBorder(
                map_else_err_str(tag_value, |value| Some(Border::Vertical(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid border vertical '{}'!", value)) )?
            )),
            "join" => objects.push(EventObject::TagJoin(
                map_or_err_str(tag_value, |value| Join::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid join '{}'!", value)) )?
            )),
            "cap" => objects.push(EventObject::TagCap(
                map_or_err_str(tag_value, |value| Cap::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid cap '{}'!", value)) )?
            )),
            "texture" => objects.push(EventObject::TagTexture(
                map_else_err_str(tag_value, |value| Some(value.to_owned()) )
                .map_err(|value| ParseError::new(&format!("Invalid texture '{}'!", value)) )?
            )),
            "texfill" => objects.push(EventObject::TagTexFill(
                map_else_err_str(tag_value, |value| {
                    let mut tokens = value.splitn(5, VALUE_SEPARATOR);
                    Some(Box::new(TexFill {
                        x0: tokens.next()?.parse().ok()?,
                        y0: tokens.next()?.parse().ok()?,
                        x1: tokens.next()?.parse().ok()?,
                        y1: tokens.next()?.parse().ok()?,
                        wrap: TextureWrapping::try_from(tokens.next()?).ok()?
                    }))
                } )
                .map_err(|value| ParseError::new(&format!("Invalid texture filling '{}'!", value)) )?
            )),
            "color" => objects.push(EventObject::TagColor(
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
                .map_err(|value| ParseError::new(&format!("Invalid color '{}'!", value)) )?
            )),
            "bordercolor" => objects.push(EventObject::TagBorderColor(
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
                .map_err(|value| ParseError::new(&format!("Invalid color '{}'!", value)) )?
            )),
            "alpha" => objects.push(EventObject::TagAlpha(
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
                .map_err(|value| ParseError::new(&format!("Invalid color '{}'!", value)) )?
            )),
            "borderalpha" => objects.push(EventObject::TagBorderAlpha(
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
                .map_err(|value| ParseError::new(&format!("Invalid color '{}'!", value)) )?
            )),
            "blur" => objects.push(EventObject::TagBlur(
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
                .map_err(|value| ParseError::new(&format!("Invalid blur '{}'!", value)) )?
            )),
            "blur-h" => objects.push(EventObject::TagBlur(
                map_else_err_str(tag_value, |value| Some(Blur::Horizontal(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid blur horizontal '{}'!", value)) )?
            )),
            "blur-v" => objects.push(EventObject::TagBlur(
                map_else_err_str(tag_value, |value| Some(Blur::Vertical(value.parse().ok()?)) )
                .map_err(|value| ParseError::new(&format!("Invalid blur vertical '{}'!", value)) )?
            )),
            "blend" => objects.push(EventObject::TagBlend(
                map_or_err_str(tag_value, |value| Blend::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid blend '{}'!", value)) )?
            )),
            "target" => objects.push(EventObject::TagTarget(
                map_or_err_str(tag_value, |value| Target::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid target '{}'!", value)) )?
            )),
            "mask-mode" => objects.push(EventObject::TagMaskMode(
                map_or_err_str(tag_value, |value| MaskMode::try_from(value) )
                .map_err(|value| ParseError::new(&format!("Invalid mask mode '{}'!", value)) )?
            )),
            "mask-clear" => objects.push(Some(EventObject::TagMaskClear)
                .filter(|_| tag_value.is_none() )
                .ok_or_else(|| ParseError::new("Mask clear has no value!") )?
            ),
            "animate" if mode.is_some() => objects.push(EventObject::TagAnimate(

                // TODO: implement animate

                /*
                map_or_err_str(tag_value, |value| {

                } )
                .map_err(|value| ParseError::new(&format!("Invalid animate '{}'!", value)) )?
                */
                Box::new(Animate {
                    time: Some((500, -1000)),
                    formula: Some("t^2".to_owned()),
                    tags: vec![
                        EventObject::TagSize(
                            42.0
                        ),
                        EventObject::TagColor(Color::Mono([
                            0, 128, 255
                        ])),
                        EventObject::TagTranslate(Translate::X(
                            99.9
                        ))
                    ]
                })
            )),
            "k" => objects.push(EventObject::TagKaraoke(
                map_or_err_str(tag_value, |value| value.parse() )
                .map_err(|value| ParseError::new(&format!("Invalid karaoke '{}'!", value)) )?
            )),
            "kset" => objects.push(EventObject::TagKaraokeSet(
                map_or_err_str(tag_value, |value| value.parse() )
                .map_err(|value| ParseError::new(&format!("Invalid karaoke set '{}'!", value)) )?
            )),
            "kcolor" => objects.push(EventObject::TagKaraokeColor(
                map_or_err_str(tag_value, |value| rgb_from_str(value) )
                .map_err(|value| ParseError::new(&format!("Invalid karaoke color '{}'!", value)) )?
            )),
            _ => return Err(ParseError::new(&format!("Invalid tag '{}'!", tag_name)))
        }
    }
    Ok(())
}
fn parse_geometries(data: &str, objects: &mut Vec<EventObject>, mode: &Mode) -> Result<(), ParseError> {
    match mode {
        Mode::Text => objects.push(EventObject::GeometryText(data.to_owned())),
        Mode::Points => {
            // Find points
            let tokens = data.split_ascii_whitespace().collect::<Vec<&str>>();
            let mut points = Vec::with_capacity(tokens.len() >> 1);
            let mut tokens = tokens.iter();
            // Collect points
            loop {
                match (tokens.next(), tokens.next()) {
                    (Some(x), Some(y)) => points.push(Point2D {
                        x: x.parse().map_err(|_| ParseError::new(&format!("Invalid X coordinate of point '{}'!", x)) )?,
                        y: y.parse().map_err(|_| ParseError::new(&format!("Invalid Y coordinate of point '{}'!", y)) )?
                    }),
                    (Some(leftover), None) => return Err(ParseError::new(&format!("Points incomplete (leftover: '{}')!", leftover))),
                    _ => break
                }
            }
            // Save points
            objects.push(EventObject::GeometryPoints(points));
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
                            x: token.parse().map_err(|_| ParseError::new(&format!("Invalid X coordinate of move '{}'!", token)) )?,
                            y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid Y coordinate of move '{}'!", token)) )?
                        })),
                        ShapeSegmentType::Line => segments.push(ShapeSegment::LineTo(Point2D {
                            x: token.parse().map_err(|_| ParseError::new(&format!("Invalid X coordinate of line '{}'!", token)) )?,
                            y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid Y coordinate of line '{}'!", token)) )?
                        })),
                        ShapeSegmentType::Curve => segments.push(ShapeSegment::CurveTo(
                            Point2D {
                                x: token.parse().map_err(|_| ParseError::new(&format!("Invalid X coordinate of curve first point '{}'!", token)) )?,
                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid Y coordinate of curve first point '{}'!", token)) )?
                            },
                            Point2D {
                                x: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid X coordinate of curve second point '{}'!", token)) )?,
                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid Y coordinate of curve second point '{}'!", token)) )?
                            },
                            Point2D {
                                x: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid X coordinate of curve third point '{}'!", token)) )?,
                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid Y coordinate of curve third point '{}'!", token)) )?
                            }
                        )),
                        ShapeSegmentType::Arc => segments.push(ShapeSegment::ArcBy(
                            Point2D {
                                x: token.parse().map_err(|_| ParseError::new(&format!("Invalid X coordinate of arc '{}'!", token)) )?,
                                y: map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid Y coordinate of arc '{}'!", token)) )?
                            },
                            map_or_err_str(tokens.next(), |token| token.parse()).map_err(|token| ParseError::new(&format!("Invalid degree of arc '{}'!", token)) )?
                        )),
                    }
                }
            }
            // Save segments
            segments.shrink_to_fit();
            objects.push(EventObject::GeometryShape(segments));
        }
    }
    Ok(())
}