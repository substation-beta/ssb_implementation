use crate::utils::pattern::*;
use std::collections::{HashMap,HashSet};


pub fn flatten_macro<'a>(macro_name: &str, history: &mut HashSet<&'a str>, macros: &'a HashMap<String, String>, flat_macros: &mut HashMap<&'a str, String>) -> Result<(), MacroError> {
    // Macro already flattened?
    if flat_macros.contains_key(macro_name) {
        return Ok(());
    }
    // Macro exists?
    let (macro_name, mut flat_macro_value) = macros.get_key_value(macro_name)
        .map(|key_value| (key_value.0.as_str(), key_value.1.to_owned()))
        .ok_or_else(|| MacroError::NotFound(macro_name.to_owned()))?;
    // Macro already in history (avoid infinite loop!)
    if history.contains(macro_name) {
        return Err(MacroError::InfiniteLoop(macro_name.to_owned()));
    } else {
        history.insert(macro_name);
    }
    // Process macro value
    while let Some(found) = MACRO_PATTERN.find(&flat_macro_value) {
        // Insert sub-macro
        let sub_macro_name = &flat_macro_value[found.start()+MACRO_INLINE_START.len()..found.end()-MACRO_INLINE_END.len()];
        if !flat_macros.contains_key(sub_macro_name) {
            flatten_macro(sub_macro_name, history, macros, flat_macros)?;
        }
        let sub_macro_location = found.start()..found.end();
        let sub_macro_value = flat_macros.get(sub_macro_name).ok_or_else(|| MacroError::NotFound(sub_macro_name.to_owned()))?;
        flat_macro_value.replace_range(sub_macro_location, sub_macro_value);
    }
    // Register flat macro
    flat_macros.insert(
        macro_name,
        flat_macro_value
    );
    // Everything alright
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum MacroError {
    NotFound(String),
    InfiniteLoop(String)
}


#[cfg(test)]
mod tests {
    use super::{flatten_macro,HashMap,HashSet,MacroError};

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
    fn compare_macro_errors() {
        assert_ne!(MacroError::InfiniteLoop("".to_owned()), MacroError::NotFound("zzz".to_owned()));
    }
}