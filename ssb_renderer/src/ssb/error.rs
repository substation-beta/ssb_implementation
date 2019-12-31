// Imports
use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result
    }
};
use crate::g2d::error::GraphicsError;


/// SSB rendering specific error type.
#[derive(Debug)]
pub struct RenderingError {
    msg: String,
    src: Option<Box<dyn Error>>
}
impl RenderingError {
    /// New error with message only.
    pub(crate) fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            src: None
        }
    }
    /// New error with message and source error.
    pub(crate) fn new_with_source<E>(msg: &str, src: E) -> Self
        where E: Error + 'static {
        Self {
            msg: msg.to_owned(),
            src: Some(Box::new(src))
        }
    }
}
impl Display for RenderingError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
        .and_then(|_| write!(f, "{}", self.source().map_or(String::new(), |src| format!("\n{}", src))))
    }
}
impl Error for RenderingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.src.as_ref().map(AsRef::as_ref)
    }
}
impl From<std::io::Error> for RenderingError {
    fn from(err: std::io::Error) -> Self {
        Self::new(&err.to_string())
    }
}
impl From<GraphicsError> for RenderingError {
    fn from(err: GraphicsError) -> Self {
        Self::new(&err.to_string())
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::RenderingError;

    #[test]
    fn rendering_error() {
        assert_eq!(RenderingError::new("easy").to_string(), "easy");
    }

    #[test]
    fn rendering_error_with_source() {
        assert_eq!(RenderingError::new_with_source("parent error", RenderingError::new("nested error")).to_string(), "parent error\nnested error");
    }

    #[test]
    fn rendering_error_from_io() {
        use std::io::{Error, ErrorKind};
        assert_eq!(RenderingError::from(Error::new(ErrorKind::PermissionDenied, "No access on filesystem!")).to_string(), "No access on filesystem!".to_owned());
    }
}