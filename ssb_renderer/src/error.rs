// Imports
use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result
    }
};
use puny2d::error::GraphicsError;


/// SSB rendering specific error type.
#[derive(Debug)]
pub struct RenderingError {
    msg: String,
    src: Box<dyn Error>
}
impl RenderingError {
    /// New error with message and source error.
    pub(crate) fn new_with_source<E>(msg: &str, src: E) -> Self
        where E: Error + 'static {
        Self {
            msg: msg.to_owned(),
            src: Box::new(src)
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
        Some(self.src.as_ref())
    }
}
impl From<std::io::Error> for RenderingError {
    fn from(err: std::io::Error) -> Self {
        Self::new_with_source("IO error!", err)
    }
}
impl From<GraphicsError> for RenderingError {
    fn from(err: GraphicsError) -> Self {
        Self::new_with_source("Graphics error!", err)
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::RenderingError;

    #[test]
    fn rendering_error_from_io() {
        use std::io::{Error, ErrorKind};
        assert_eq!(RenderingError::from(Error::new(ErrorKind::PermissionDenied, "No access on filesystem!")).to_string(), "IO error!\nNo access on filesystem!".to_owned());
    }
}