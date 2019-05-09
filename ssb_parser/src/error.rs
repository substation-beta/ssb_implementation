// Imports
use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
        Result
    }
};

// Custom error
#[derive(Debug)]
pub struct ParseError {
    msg: String,
    span: Option<((usize, usize), (usize, usize))>,
    src: Option<Box<Error>>
}
impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(src) = &self.src {
            Some(src.as_ref())
        } else {
            None
        }
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(span) = self.span {
            let ((x0, y0), (x1, y1)) = span;
            if x0 == x1 && y0 == y1 {
                write!(f, "{} <{}:{}>", self.msg, x0, y0)
            } else {
                write!(f, "{} <{}:{}-{}:{}>", self.msg, x0, y0, x1, y1)
            }
        } else {
            write!(f, "{}", self.msg)
        }
    }
}
impl<R> From<pest::error::Error<R>> for ParseError
    where R: Debug {
    fn from(err: pest::error::Error<R>) -> Self {
        Self::new_with_span(
            &match err.variant {
                pest::error::ErrorVariant::ParsingError{positives, negatives} => format!("Expected {:?}, found {:?}", positives, negatives),
                pest::error::ErrorVariant::CustomError{message} => message
            },
            match err.line_col {
                pest::error::LineColLocation::Pos(pos) => (pos, pos),
                pest::error::LineColLocation::Span(start, stop) => (start, stop)
            }
        )
    }
}
impl ParseError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            span: None,
            src: None
        }
    }
    pub fn new_with_span(msg: &str, span: ((usize, usize), (usize, usize))) -> Self {
        Self {
            msg: msg.to_owned(),
            span: Some(span),
            src: None
        }
    }
    pub fn new_with_source(msg: &str, span: Option<((usize, usize), (usize, usize))>, src: Box<Error>) -> Self {
        Self {
            msg: msg.to_owned(),
            span: span,
            src: Some(src)
        }
    }
}

// Error identifiers
#[derive(Debug)]
pub enum MacroError {
    NotFound(String),
    InfiniteLoop(String)
}