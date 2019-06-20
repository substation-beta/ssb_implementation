// Imports
use std::convert::TryFrom;


// General
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub struct Point2D {
    pub x: Coordinate,
    pub y: Coordinate
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub struct Point3D {
    pub x: Coordinate,
    pub y: Coordinate,
    pub z: Coordinate
}
pub type Coordinate = f32;
pub type Degree = f64;


// Objects
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum EventObject {
    GeometryShape(Vec<ShapeSegment>),
    GeometryPoints(Vec<Point2D>),
    GeometryText(String),
    TagFont(String),
    TagSize(f32),
    TagBold(bool),
    TagItalic(bool),
    TagUnderline(bool),
    TagStrikeout(bool),
    TagPosition(Point3D),
    TagAlignment(Alignment),
    TagMargin(Margin),
    TagWrapStyle(WrapStyle),
    TagDirection(Direction),
    TagSpace(Space),
    TagRotate(Rotate),
    TagScale(Scale),
    TagTranslate(Translate),
    TagShear(Shear),
    TagMatrix(Box<[Degree;16]>),
    TagIdentity,
    TagBorder(Border),
    TagJoin(Join),
    TagCap(Cap),
    TagTexture(String),
    TagTexFill{
        x0: Degree,
        y0: Degree,
        x1: Degree,
        y1: Degree,
        wrap: TextureWrapping
    },
    TagColor(Color),
    TagBorderColor(Color),
    TagAlpha(Alpha),
    TagBorderAlpha(Alpha),
    TagBlur(Blur),
    TagBlend(Blend),
    TagTarget(Target),
    TagMaskMode(MaskMode),
    TagMaskClear,
    TagAnimate(Box<Animate>),
    TagKaraoke(u32),
    TagKaraokeSet(i32),
    TagKaraokeColor([u8;3])
}


// Object properties
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum ShapeSegment {
    MoveTo(Point2D),
    LineTo(Point2D),
    CurveTo(Point2D, Point2D, Point2D),
    ArcBy(Point2D, Degree),
    Close
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Alignment {
    Numpad(Numpad),
    Offset(Point2D)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Numpad {
    TopLeft, TopCenter, TopRight,
    MiddleLeft, MiddleCenter, MiddleRight,
    BottomLeft, BottomCenter, BottomRight
}
impl TryFrom<u8> for Numpad {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Numpad::BottomLeft),
            2 => Ok(Numpad::BottomCenter),
            3 => Ok(Numpad::BottomRight),
            4 => Ok(Numpad::MiddleLeft),
            5 => Ok(Numpad::MiddleCenter),
            6 => Ok(Numpad::MiddleRight),
            7 => Ok(Numpad::TopLeft),
            8 => Ok(Numpad::TopCenter),
            9 => Ok(Numpad::TopRight),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Margin {
    All(Coordinate, Coordinate, Coordinate, Coordinate),
    Top(Coordinate),
    Right(Coordinate),
    Bottom(Coordinate),
    Left(Coordinate)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum WrapStyle {
    Space,
    Character,
    NoWrap
}
impl TryFrom<&str> for WrapStyle {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "space" => Ok(WrapStyle::Space),
            "character" => Ok(WrapStyle::Character),
            "nowrap" => Ok(WrapStyle::NoWrap),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop
}
impl TryFrom<&str> for Direction {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ltr" => Ok(Direction::LeftToRight),
            "rtl" => Ok(Direction::RightToLeft),
            "ttb" => Ok(Direction::TopToBottom),
            "btt" => Ok(Direction::BottomToTop),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Space {
    All(Coordinate, Coordinate),
    Horizontal(Coordinate),
    Vertical(Coordinate)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Rotate {
    All(Degree, Degree, Degree),
    X(Degree),
    Y(Degree),
    Z(Degree)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Scale {
    All(Degree, Degree, Degree),
    X(Degree),
    Y(Degree),
    Z(Degree)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Translate {
    All(Coordinate, Coordinate, Coordinate),
    X(Coordinate),
    Y(Coordinate),
    Z(Coordinate)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Shear {
    All(Degree, Degree),
    X(Degree),
    Y(Degree)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Border {
    All(Coordinate, Coordinate),
    Horizontal(Coordinate),
    Vertical(Coordinate)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Join {
    Round,
    Bevel,
    Miter
}
impl TryFrom<&str> for Join {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "round" => Ok(Join::Round),
            "bevel" => Ok(Join::Bevel),
            "miter" => Ok(Join::Miter),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Cap {
    Round,
    Butt,
    Square
}
impl TryFrom<&str> for Cap {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "round" => Ok(Cap::Round),
            "butt" => Ok(Cap::Butt),
            "square" => Ok(Cap::Square),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum TextureWrapping {
    Pad,
    Clamp,
    Repeat,
    Mirror
}
impl TryFrom<&str> for TextureWrapping {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "pad" => Ok(TextureWrapping::Pad),
            "clamp" => Ok(TextureWrapping::Clamp),
            "repeat" => Ok(TextureWrapping::Repeat),
            "mirror" => Ok(TextureWrapping::Mirror),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Color {
    Mono([u8;3]),
    Linear([[u8;3];2]),
    LinearWithStop([[u8;3];3]),
    Corners([[u8;3];4]),
    CornersWithStop([[u8;3];5])
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Alpha {
    Mono(u8),
    Linear([u8;2]),
    LinearWithStop([u8;3]),
    Corners([u8;4]),
    CornersWithStop([u8;5])
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Blur {
    All(Coordinate, Coordinate),
    Horizontal(Coordinate),
    Vertical(Coordinate)
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Blend {
    Add,
    Subtract,
    Multiply,
    Invert,
    Difference,
    Screen
}
impl TryFrom<&str> for Blend {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" => Ok(Blend::Add),
            "subtract" => Ok(Blend::Subtract),
            "multiply" => Ok(Blend::Multiply),
            "invert" => Ok(Blend::Invert),
            "difference" => Ok(Blend::Difference),
            "screen" => Ok(Blend::Screen),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum Target {
    Frame,
    Mask
}
impl TryFrom<&str> for Target {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "frame" => Ok(Target::Frame),
            "mask" => Ok(Target::Mask),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub enum MaskMode {
    Normal,
    Invert
}
impl TryFrom<&str> for MaskMode {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "normal" => Ok(MaskMode::Normal),
            "invert" => Ok(MaskMode::Invert),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize))]
pub struct Animate {
    pub time: Option<(i32, i32)>,
    pub formula: Option<String>,
    pub tags: Vec<EventObject>
}


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        use super::{Numpad, WrapStyle, Direction, Join, Cap, TextureWrapping, Blend, Target, MaskMode, TryFrom};
        assert_eq!(Numpad::try_from(7u8), Ok(Numpad::TopLeft));
        assert_eq!(WrapStyle::try_from("character"), Ok(WrapStyle::Character));
        assert_eq!(Direction::try_from("ttb"), Ok(Direction::TopToBottom));
        assert_eq!(Join::try_from("bevel"), Ok(Join::Bevel));
        assert_eq!(Cap::try_from("butt"), Ok(Cap::Butt));
        assert_eq!(TextureWrapping::try_from("mirror"), Ok(TextureWrapping::Mirror));
        assert_eq!(Blend::try_from("invert"), Ok(Blend::Invert));
        assert_eq!(Target::try_from("mask"), Ok(Target::Mask));
        assert_eq!(MaskMode::try_from("invert"), Ok(MaskMode::Invert));
    }
}