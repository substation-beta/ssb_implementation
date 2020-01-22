pub trait OptionExt<'a> {
    fn map_or_err_str<F,U,E>(self, op: F) -> Result<U,&'a str>
        where F: FnOnce(&str) -> Result<U,E>;
    fn map_else_err_str<F,U>(self, op: F) -> Result<U,&'a str>
        where F: FnOnce(&str) -> Option<U>;
}
impl<'a, T: AsRef<str> + ?Sized> OptionExt<'a> for Option<&'a T> {
    fn map_or_err_str<F,U,E>(self, op: F) -> Result<U,&'a str>
        where F: FnOnce(&str) -> Result<U,E> {
        self.map_or(Err(""), |value| op(value.as_ref()).map_err(|_| value.as_ref() ))
    }
    fn map_else_err_str<F,U>(self, op: F) -> Result<U,&'a str>
        where F: FnOnce(&str) -> Option<U> {
        self.map_or(Err(""), |value| op(value.as_ref()).ok_or_else(|| value.as_ref() ))
    }
}


#[cfg(test)]
mod tests {
    use super::OptionExt;

    #[test]
    fn map_err_str() {
        assert_eq!(Some("123").map_or_err_str(|value| value.parse()), Ok(123));
        assert_eq!(Some("987a").map_else_err_str(|value| value.parse::<i32>().ok()), Err("987a"));
    }
}