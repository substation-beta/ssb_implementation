// Imports
use super::{
    error::*
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap,HashSet}
};


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
pub const VALUE_SEPARATOR: &str = ",";
pub const EVENT_SEPARATOR: &str = "|";
pub const TRIGGER_SEPARATOR: &str = "-";
pub const TAG_START: &str = "[";
pub const TAG_START_CHAR: char = '[';
pub const TAG_END: &str = "]";
pub const TAG_END_CHAR: char = ']';
lazy_static! {
    pub static ref MACRO_PATTERN: Regex = Regex::new("\\$\\{([a-zA-Z0-9_-]+)\\}").unwrap();
    static ref TIMESTAMP_PATTERN: Regex = Regex::new("^(?:(?:(?P<H>\\d{0,2}):(?P<HM>[0-5]?\\d?):)|(?:(?P<M>[0-5]?\\d?):))?(?:(?P<S>[0-5]?\\d?)\\.)?(?P<MS>\\d{0,3})$").unwrap();
}


// Utilities
pub fn parse_timestamp(timestamp: &str) -> Result<u32,()> {
    // Milliseconds factors
    const MS_2_MS: u32 = 1;
    const S_2_MS: u32 = MS_2_MS * 1000;
    const M_2_MS: u32 = S_2_MS * 60;
    const H_2_MS: u32 = M_2_MS * 60;
    // Calculate time in milliseconds
    let mut ms = 0u32;
    let captures = TIMESTAMP_PATTERN.captures(timestamp).ok_or_else(|| ())?;
    for (unit, factor) in &[("MS", MS_2_MS), ("S", S_2_MS), ("M", M_2_MS), ("HM", M_2_MS), ("H", H_2_MS)] {
        if let Some(unit_value) = captures.name(unit) {
            if unit_value.start() != unit_value.end() { // Not empty
                ms += unit_value.as_str().parse::<u32>().map_err(|_| ())? * factor;
            }
        }
    }
    // Return time
    Ok(ms)
}

pub fn flatten_macro(macro_name: &str, history: &mut HashSet<String>, macros: &HashMap<String, String>, flat_macros: &mut HashMap<String, String>) -> Result<(), MacroError> {
    // Macro already flattened?
    if flat_macros.contains_key(macro_name) {
        return Ok(());
    }
    // Macro already in history (avoid infinite loop!)
    if history.contains(macro_name) {
        return Err(MacroError::InfiniteLoop(macro_name.to_owned()));
    } else {
        history.insert(macro_name.to_owned());
    }
    // Process macro value
    let mut flat_macro_value = macros.get(macro_name).ok_or(MacroError::NotFound(macro_name.to_owned()))?.clone();
    while let Some(found) = MACRO_PATTERN.find(&flat_macro_value) {
        // Insert sub-macro
        let sub_macro_name = &flat_macro_value[found.start()+2..found.end()-1];
        if !flat_macros.contains_key(sub_macro_name) {
            flatten_macro(&sub_macro_name, history, macros, flat_macros)?;
        }
        flat_macro_value.replace_range(
            found.start()..found.end(),
            flat_macros.get(sub_macro_name).ok_or(MacroError::NotFound(sub_macro_name.to_owned()))?
        );
    }
    // Register flat macro
    flat_macros.insert(
        macro_name.to_owned(),
        flat_macro_value
    );
    // Everything alright
    Ok(())
}

pub struct TagGeometryIterator {
    text: String,
    pos: usize
}
impl TagGeometryIterator {
    pub fn new(source: &str) -> Self {
        Self {
            text: source.replace("\\\\", "\x1B").replace(&("\\".to_owned() + TAG_START), "\x02").replace(&("\\".to_owned() + TAG_END), "\x03").replace("\\n", "\n").replace("\x1B", "\\"),
            pos: 0
        }
    }
}
impl Iterator for TagGeometryIterator {
    type Item = (bool, String);
    fn next(&mut self) -> Option<Self::Item> {
        // Remaining of source
        let text = &self.text[self.pos..];
        // End of source reached?
        if text.is_empty() {
            return None;
        }
        // Match tag or geometry
        let is_tag;
        let text_chunk;
        if text.starts_with(TAG_START) {
            is_tag = true;
            // Till tag end (considers nested tags)
            let mut tag_open_count = 0usize;
            if let Some(end_pos) = text.char_indices().skip(1).find(|c| match c.1 {
                TAG_START_CHAR => {tag_open_count+=1; false},
                TAG_END_CHAR => if tag_open_count == 0 {true} else {tag_open_count-=1; false}
                _ => false
            }).map(|c| c.0) {
                self.pos += end_pos + TAG_END.len();
                text_chunk = &text[TAG_START.len()..end_pos];
            // Till end
            } else {
                self.pos += text.len();
                text_chunk = &text[TAG_START.len()..];
            }
        } else {
            is_tag = false;
            // Till tag start
            if let Some(start_pos) = text.find(TAG_START) {
                self.pos += start_pos;
                text_chunk = &text[..start_pos];
            // Till end
            } else {
                self.pos += text.len();
                text_chunk = text;
            }
        }
        // Return tag or geometry with unescaped characters
        Some((is_tag, text_chunk.replace("\x02", TAG_START).replace("\x03", TAG_END)))
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::{
        parse_timestamp,
        flatten_macro,
        TagGeometryIterator,
        super::error::MacroError,
        HashMap,
        HashSet
    };

    #[test]
    fn parse_timestamp_various() {
        assert_eq!(parse_timestamp(""), Ok(0));
        assert_eq!(parse_timestamp("1:2.3"), Ok(62_003));
        assert_eq!(parse_timestamp("59:59.999"), Ok(3_599_999));
        assert_eq!(parse_timestamp("1::.1"), Ok(3_600_001));
    }

    #[test]
    fn flatten_macro_success() {
        // Test data
        let mut macros = HashMap::new();
        macros.insert("a".to_owned(), "Hello ${b} test!".to_owned());
        macros.insert("b".to_owned(), "fr${c}".to_owned());
        macros.insert("c".to_owned(), "om".to_owned());
        let mut flat_macros = HashMap::new();
        // Test execution
        flatten_macro("a", &mut HashSet::new(), &macros, &mut flat_macros).unwrap();
        assert_eq!(flat_macros.get("a").unwrap(), "Hello from test!");
    }
    #[test]
    fn flatten_macro_infinite() {
        // Test data
        let mut macros = HashMap::new();
        macros.insert("a".to_owned(), "foo ${b}".to_owned());
        macros.insert("b".to_owned(), "${a} bar".to_owned());
        // Test execution
        assert_eq!(flatten_macro("a", &mut HashSet::new(), &macros, &mut HashMap::new()).unwrap_err(), MacroError::InfiniteLoop("a".to_owned()));
    }
    #[test]
    fn flatten_macro_notfound() {
        assert_eq!(flatten_macro("x", &mut HashSet::new(), &HashMap::new(), &mut HashMap::new()).unwrap_err(), MacroError::NotFound("x".to_owned()));
    }

    #[test]
    fn tag_geometry_iter() {
        let mut iter = TagGeometryIterator::new("[tag1][tag2=[inner_tag]]geometry1\\[geometry1_continue\\\\[tag3]geometry2\\n[tag4");
        assert_eq!(iter.next(), Some((true, "tag1".to_owned())));
        assert_eq!(iter.next(), Some((true, "tag2=[inner_tag]".to_owned())));
        assert_eq!(iter.next(), Some((false, "geometry1[geometry1_continue\\".to_owned())));
        assert_eq!(iter.next(), Some((true, "tag3".to_owned())));
        assert_eq!(iter.next(), Some((false, "geometry2\n".to_owned())));
        assert_eq!(iter.next(), Some((true, "tag4".to_owned())));
        assert_eq!(iter.next(), None);
    }
}