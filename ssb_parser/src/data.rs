// Imports
use std::collections::HashMap;


// Sub types
#[derive(Debug)]
pub enum View {
    Perspective,
    Orthogonal
}
impl View {
    pub fn from_str(str: &str) -> Result<Self,()> {
        match str {
            "perspective" => Ok(View::Perspective),
            "orthogonal" => Ok(View::Orthogonal),
            _ => Err(())
        }
    }
}
#[derive(Debug)]
pub struct Event {
    pub script_line: u32,
    pub trigger: EventTrigger,
    pub macro_: Option<String>,
    pub note: Option<String>,
    pub data: String
}
#[derive(Debug)]
pub enum EventTrigger {
    Id(String),
    Time((u32,u32))
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FontFace {
    family: String,
    style: FontStyle
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic
}
impl FontStyle {
    pub fn from_str(str: &str) -> Result<Self,()> {
        match str {
            "regular" => Ok(FontStyle::Regular),
            "bold" => Ok(FontStyle::Bold),
            "italic" => Ok(FontStyle::Italic),
            "bold-italic" => Ok(FontStyle::BoldItalic),
            _ => Err(())
        }
    }
}
pub type FontData = String;
pub type TextureId = String;
#[derive(Debug)]
pub enum TextureData {
    Url(String),
    Raw(String)
}

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