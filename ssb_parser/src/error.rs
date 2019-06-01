// Imports
use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result
    }
};


// Custom error
#[derive(Debug)]
pub struct ParseError {
    msg: String,
    pos: Option<(usize, usize)>,
    src: Option<Box<dyn Error>>
}
impl ParseError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            pos: None,
            src: None
        }
    }
    pub fn new_with_pos(msg: &str, pos: (usize, usize)) -> Self {
        Self {
            msg: msg.to_owned(),
            pos: Some(pos),
            src: None
        }
    }
    pub fn new_with_source<E>(msg: &str, pos: (usize, usize), src: E) -> Self
        where E: Error + 'static {
        Self {
            msg: msg.to_owned(),
            pos: Some(pos),
            src: Some(Box::new(src))
        }
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.pos
            .map(|pos| write!(f, "{} <{}:{}>", self.msg, pos.0, pos.1))
            .unwrap_or_else(|| write!(f, "{}", self.msg))
    }
}
impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.src.as_ref().map(|src| src.as_ref())
    }
}
impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        Self::new(err.description())
    }
}


// Error identifiers
#[derive(Debug, PartialEq)]
pub enum MacroError {
    NotFound(String),
    InfiniteLoop(String)
}