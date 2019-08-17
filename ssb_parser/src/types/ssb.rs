// Imports
use std::fmt;
use std::convert::TryFrom;
use super::objects::EventObject;


// Data minor types
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub struct Event {
    pub trigger: EventTrigger,
    pub macro_name: Option<String>,
    pub note: Option<String>,
    pub data: String,
    pub data_location: (usize,usize)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub struct EventRender {
    pub trigger: EventTrigger,
    pub objects: Vec<EventObject>
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum EventTrigger {
    Id(String),
    Time((u32,u32))
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum View {
    Perspective,
    Orthogonal
}
impl TryFrom<&str> for View {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "perspective" => Ok(Self::Perspective),
            "orthogonal" => Ok(Self::Orthogonal),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub struct FontFace {
    pub family: String,
    pub style: FontStyle
}
impl fmt::Display for FontFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.family, self.style)
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic
}
impl TryFrom<&str> for FontStyle {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "regular" => Ok(Self::Regular),
            "bold" => Ok(Self::Bold),
            "italic" => Ok(Self::Italic),
            "bold-italic" => Ok(Self::BoldItalic),
            _ => Err(())
        }
    }
}
pub type FontData = Vec<u8>;
pub type TextureId = String;
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum TextureDataVariant {
    Raw(TextureData),
    Url(String)
}
pub type TextureData = Vec<u8>;


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        use super::{View, FontStyle, TryFrom};
        assert_eq!(View::try_from("orthogonal"), Ok(View::Orthogonal));
        assert_eq!(View::try_from("perspective"), Ok(View::Perspective));
        assert_eq!(View::try_from("fuzzy"), Err(()));
        assert_eq!(FontStyle::try_from("regular"), Ok(FontStyle::Regular));
        assert_eq!(FontStyle::try_from("bold"), Ok(FontStyle::Bold));
        assert_eq!(FontStyle::try_from("italic"), Ok(FontStyle::Italic));
        assert_eq!(FontStyle::try_from("bold-italic"), Ok(FontStyle::BoldItalic));
        assert_eq!(FontStyle::try_from("ultra-bold"), Err(()));
    }
}