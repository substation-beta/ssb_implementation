// Imports
use std::error::Error;
use std::fmt;
use std::sync::mpsc::{SendError, RecvError};
use super::safe::GetError;

// Structure
#[derive(Debug)]
pub struct GlError {
    message: String,
    source: Option<Box<Error>>
}

// Implementation
impl GlError {
    pub fn new(message: &str) -> Self {
        Self{ message: message.to_string(), source: None }
    }
    pub fn new_with_source(message: &str, source: Box<Error>) -> Self {
        Self{ message: message.to_string(), source: Some(source) }
    }
    pub fn from_gl() -> Option<Self> {
        let error_code = GetError();
        if error_code == gl32::NO_ERROR {
            None
        } else {
            Some(Self::new(&format!("Error code: {:#X} (see https://www.khronos.org/opengl/wiki/OpenGL_Error#Meaning_of_errors )", error_code)))
        }
    }
}

// Extensions
impl Error for GlError {
    fn description(&self) -> &str {
        &self.message
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(ref source) = self.source {
            Some(source.as_ref())
        } else {
            None
        }
    }
}
impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(source) = self.source() {
            write!(f, "{} - Source: {}", self.description(), source)
        } else {
            write!(f, "{}", self.description())
        }
    }
}
impl<DataType> From<SendError<DataType>> for GlError
    where DataType: std::marker::Send + 'static {
    fn from(error: SendError<DataType>) -> Self {
        Self::new_with_source("Send error!", Box::new(error))
    }
}
impl From<RecvError> for GlError {
    fn from(error: RecvError) -> Self {
        Self::new_with_source("Receive error!", Box::new(error))
    }
}