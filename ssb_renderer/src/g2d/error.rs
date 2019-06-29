// Imports
use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result
    }
};


/// Graphics processing specific error type.
#[derive(Debug)]
pub struct GraphicsError {
    msg: String
}
impl GraphicsError {
    /// New error with message.
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned()
        }
    }
}
impl Display for GraphicsError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.msg)
    }
}
impl Error for GraphicsError {}


// Tests
#[cfg(test)]
mod tests {
    use super::GraphicsError;

    #[test]
    fn graphics_error() {
        assert_eq!(GraphicsError::new("Not enough!").to_string(), "Not enough!");
    }
}