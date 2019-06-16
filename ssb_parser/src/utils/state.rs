// Imports
use std::convert::TryFrom;


// Enums
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
pub enum ShapeSegmentType {
    Move,
    Line,
    Curve,
    Arc
}
impl Default for ShapeSegmentType {
    fn default() -> Self {
        ShapeSegmentType::Move
    }
}


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        use super::{Section, Mode, TryFrom};
        assert_eq!(Section::try_from("#Events"), Ok(Section::Events));
        assert_eq!(Section::try_from("#Event"), Err(()));
        assert_eq!(Mode::try_from("shape"), Ok(Mode::Shape));
        assert_eq!(Mode::try_from("lines"), Err(()));
    }
}