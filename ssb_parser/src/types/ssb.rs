// Imports
use std::fmt;
use std::convert::TryFrom;
use super::{
    geometries::EventGeometry,
    tags::EventTag
};


// Data minor types
#[derive(Debug)]
pub struct Event {
    pub trigger: EventTrigger,
    pub macro_name: Option<String>,
    pub note: Option<String>,
    pub data: String,
    pub data_location: (usize,usize)
}
#[derive(Debug)]
pub struct EventRender {
    pub trigger: EventTrigger,
    pub objects: Vec<EventObject>
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventTrigger {
    Id(String),
    Time((u32,u32))
}

#[derive(Debug, PartialEq, Clone)]
pub enum View {
    Perspective,
    Orthogonal
}
impl TryFrom<&str> for View {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "perspective" => Ok(View::Perspective),
            "orthogonal" => Ok(View::Orthogonal),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
            "regular" => Ok(FontStyle::Regular),
            "bold" => Ok(FontStyle::Bold),
            "italic" => Ok(FontStyle::Italic),
            "bold-italic" => Ok(FontStyle::BoldItalic),
            _ => Err(())
        }
    }
}
pub type FontData = Vec<u8>;

pub type TextureId = String;
pub type TextureData = Vec<u8>;


// Object types
#[derive(Debug)]
pub enum EventObject {
    Geometry(EventGeometry),
    Tag(EventTag)
}
#[derive(Debug)]
pub struct Point2D {
    pub x: Coordinate,
    pub y: Coordinate
}
#[derive(Debug)]
pub struct Point3D {
    pub x: Coordinate,
    pub y: Coordinate,
    pub z: Coordinate
}
pub type Coordinate = f32;


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        use super::{View, FontStyle, TryFrom};
        assert_eq!(View::try_from("orthogonal").expect("View instance expected!"), View::Orthogonal);
        assert_eq!(FontStyle::try_from("bold-italic").expect("FontStyle instance expected!"), FontStyle::BoldItalic);
    }
}