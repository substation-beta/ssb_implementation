// Imports
use lazy_static::lazy_static;
use regex::{Regex,escape};


// Constants
pub const INFO_TITLE_KEY: &str = "Title: ";
pub const INFO_AUTHOR_KEY: &str = "Author: ";
pub const INFO_DESCRIPTION_KEY: &str = "Description: ";
pub const INFO_VERSION_KEY: &str = "Version: ";
pub const KEY_SUFFIX: &str = ": ";
pub const TARGET_WIDTH_KEY: &str = "Width: ";
pub const TARGET_HEIGHT_KEY: &str = "Height: ";
pub const TARGET_DEPTH_KEY: &str = "Depth: ";
pub const TARGET_VIEW_KEY: &str = "View: ";
pub const RESOURCES_FONT_KEY: &str = "Font: ";
pub const RESOURCES_TEXTURE_KEY: &str = "Texture: ";
pub const MACRO_INLINE_START: &str = "${";
pub const MACRO_INLINE_END: &str = "}";
pub const VALUE_SEPARATOR: char = ',';
pub const EVENT_SEPARATOR: char = '|';
pub const TRIGGER_SEPARATOR: char = '-';
pub const TAG_START: &str = "[";
pub const TAG_START_CHAR: char = '[';
pub const TAG_END: &str = "]";
pub const TAG_END_CHAR: char = ']';
pub const TAG_SEPARATOR: char = ';';
pub const TAG_ASSIGN: char = '=';

// Statics
lazy_static! {
    pub static ref MACRO_PATTERN: Regex = Regex::new(&(escape(MACRO_INLINE_START) + "([a-zA-Z0-9_-]+)" + &escape(MACRO_INLINE_END))).unwrap();
    pub static ref TIMESTAMP_PATTERN: Regex = Regex::new("^(?:(?:(?P<H>\\d{0,2}):(?P<HM>[0-5]?\\d?):)|(?:(?P<M>[0-5]?\\d?):))?(?:(?P<S>[0-5]?\\d?)\\.)?(?P<MS>\\d{0,3})$").unwrap();
}