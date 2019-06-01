// Imports
use std::fmt;
use std::convert::TryFrom;


// Sub types
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


// State
#[derive(Debug, PartialEq, Clone)]
pub enum Section {
    Info,
    Target,
    Macros,
    Events,
    Resources
}
impl TryFrom<&str> for Section {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "#Info" => Ok(Section::Info),
            "#Target" => Ok(Section::Target),
            "#Macros" => Ok(Section::Macros),
            "#Events" => Ok(Section::Events),
            "#Resources" => Ok(Section::Resources),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Text,
    Points,
    Shape
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Text
    }
}
impl TryFrom<&str> for Mode {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "text" => Ok(Mode::Text),
            "points" => Ok(Mode::Points),
            "shape" => Ok(Mode::Shape),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TextureDataType {
    Raw,
    Url
}
impl TryFrom<&str> for TextureDataType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "data" => Ok(TextureDataType::Raw),
            "url" => Ok(TextureDataType::Url),
            _ => Err(())
        }
    }
}


// Object types
#[derive(Debug)]
pub enum EventObject {
    Geometry(EventGeometry),
    Tag(EventTag)
}

#[derive(Debug)]
pub enum EventGeometry {
    Shape(Vec<ShapeSegment>),
    Points(Vec<Point2D>),
    Text(String)
}
#[derive(Debug)]
pub enum ShapeSegment {
    MoveTo(Point2D),
    LineTo(Point2D),
    CurveTo(Point2D, Point2D, Point2D),
    ArcBy(Point2D, f64),
    Close
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

#[derive(Debug)]
pub enum EventTag {
    Font(String),
    Size(f32),
    Bold(bool),
    Italic(bool),
    Underline(bool),
    Strikeout(bool),
    Position(Point3D),
    Alignment(Alignment)

    // TODO

}
#[derive(Debug)]
pub enum Alignment {
    Numpad(Numpad),
    Offset(Point2D)
}
#[derive(Debug)]
pub enum Numpad {
    BottomLeft = 1, BottomCenter = 2, BottomRight = 3,
    MiddleLeft = 4, MiddleCenter = 5, MiddleRight = 6,
    TopLeft = 7, TopCenter = 8, TopRight = 9
}


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        use super::{View, FontStyle, Section, Mode, TryFrom};
        assert_eq!(View::try_from("orthogonal").expect("View instance expected!"), View::Orthogonal);
        assert_eq!(FontStyle::try_from("bold-italic").expect("FontStyle instance expected!"), FontStyle::BoldItalic);
        assert_eq!(Section::try_from("#Events").expect("Section instance expected!"), Section::Events);
        assert_eq!(Mode::try_from("shape").expect("Mode instance expected!"), Mode::Shape);
    }
}