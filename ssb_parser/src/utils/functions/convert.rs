use crate::utils::pattern::TIMESTAMP_PATTERN;


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

pub fn bool_from_str(text: &str) -> Result<bool,()> {
    match text {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => Err(())
    }
}

pub fn alpha_from_str(text: &str) -> Result<u8,()> {
    match text.len() {
        1..=2 => u8::from_str_radix(text, 16).map_err(|_| () ),
        _ => Err(())
    }
}
pub fn rgb_from_str(text: &str) -> Result<[u8;3],()> {
    match text.len() {
        1..=6 => u32::from_str_radix(text, 16).map(|value| {let bytes = value.to_le_bytes(); [bytes[2], bytes[1], bytes[0]]} ).map_err(|_| () ),
        _ => Err(()),
    }
}


#[cfg(test)]
mod tests {
    use super::{
        parse_timestamp,
        bool_from_str,
        alpha_from_str,
        rgb_from_str
    };

    #[test]
    fn parse_timestamp_various() {
        assert_eq!(parse_timestamp(""), Ok(0));
        assert_eq!(parse_timestamp("1:2.3"), Ok(62_003));
        assert_eq!(parse_timestamp("59:59.999"), Ok(3_599_999));
        assert_eq!(parse_timestamp("1::.1"), Ok(3_600_001));
    }

    #[test]
    fn parse_bool() {
        assert_eq!(bool_from_str("y"), Ok(true));
        assert_eq!(bool_from_str("n"), Ok(false));
        assert_eq!(bool_from_str("no"), Err(()));
    }

    #[test]
    fn parse_rgb_alpha() {
        assert_eq!(alpha_from_str(""), Err(()));
        assert_eq!(alpha_from_str("A"), Ok(10));
        assert_eq!(alpha_from_str("C1"), Ok(193));
        assert_eq!(alpha_from_str("1FF"), Err(()));
        assert_eq!(rgb_from_str(""), Err(()));
        assert_eq!(rgb_from_str("1FF"), Ok([0, 1, 255]));
        assert_eq!(rgb_from_str("808080"), Ok([128, 128, 128]));
        assert_eq!(rgb_from_str("FFFF01"), Ok([255, 255, 1]));
        assert_eq!(rgb_from_str("1FFFFFF"), Err(()));
    }
}