pub fn map_or_err_str<T,F,U,E>(option: Option<& T>, op: F) -> Result<U,&str>
    where T: AsRef<str> + ?Sized,
        F: FnOnce(&str) -> Result<U,E> {
    option.map_or(Err(""), |value| op(value.as_ref()).map_err(|_| value.as_ref() ))
}
pub fn map_else_err_str<T,F,U>(option: Option<&T>, op: F) -> Result<U,&str>
    where T: AsRef<str> + ?Sized,
        F: FnOnce(&str) -> Option<U> {
    option.map_or(Err(""), |value| op(value.as_ref()).ok_or_else(|| value.as_ref() ))
}


#[cfg(test)]
mod tests {
    use super::{map_or_err_str,map_else_err_str};

    #[test]
    fn map_err_str() {
        assert_eq!(map_or_err_str(Some("123"), |value| value.parse()), Ok(123));
        assert_eq!(map_else_err_str(Some("987a"), |value| value.parse::<i32>().ok()), Err("987a"));
    }
}