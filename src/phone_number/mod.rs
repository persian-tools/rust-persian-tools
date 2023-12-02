pub mod operators;

use regex::Regex;

const MOBILE_REGEX: &str = r#"^(\+98|98|0098|0)?9(\d{2})\d{7}$"#;
pub const PREFIXES: [&str; 4] = ["+98", "98", "0098", "0"];

/// This is a simple function that checks if a phone number valid or not
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::is_phone_valid;
///
/// assert_eq!(is_phone_valid("00989122221811"),true);
/// assert_eq!(is_phone_valid("09185371111"),true);
/// assert_eq!(is_phone_valid("20989122221811"),false);
/// ```
pub fn is_phone_valid(phone_number: &str) -> bool {
    let regex = Regex::new(MOBILE_REGEX).unwrap();
    regex.is_match(phone_number)
}

/// returns phone prefix for example +98 98 based on given phone number
/// This function returns an `Option<&str>`, where `Some(prefix)` contains the extracted prefix,
/// and `None` is returned if no valid prefix is found.
///
/// # Warning
/// This function is desgined to only works for Iran phone number prefixes ("+98", "98", "0098", "0")
///
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::get_phone_prefix;
///
/// assert_eq!(get_phone_prefix("00989122221811"),Some("0098"));
/// assert_eq!(get_phone_prefix("09122221811"),Some("0"));
/// assert_eq!(get_phone_prefix("29122221811"),None);
/// ```
pub fn get_phone_prefix(phone_number: &str) -> Option<&str> {
    PREFIXES
        .into_iter()
        .find(|&prefix| phone_number.starts_with(prefix))
}

/// replaces current phone number prefix with your desired prefix
///
/// # Warning
/// if phone number is not valid it would return None
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::phone_number_normalizer;
///
/// assert_eq!(phone_number_normalizer("00989022002580" , "+98") , Some("+989022002580".to_string()));
/// assert_eq!(phone_number_normalizer("9191282819921" , "0") , None);
/// ```
pub fn phone_number_normalizer(phone_number: &str, new_prefix: &str) -> Option<String> {
    if !is_phone_valid(phone_number) {
        return None;
    }

    if let Some(prefix) = get_phone_prefix(phone_number) {
        let (_, splited) = phone_number.split_at(prefix.len());
        return Some(format!("{new_prefix}{splited}"));
    }

    Some(format!("{new_prefix}{phone_number}"))
}

/// returns operator prefix of phone number (919,912,...)
///
/// # Warning
/// if phone number is not valid it would return None
///
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::get_operator_prefix;
///
/// assert_eq!(get_operator_prefix("00989013708555") , Some("901"));
/// assert_eq!(get_operator_prefix("00988013708555") , None);
/// ```
pub fn get_operator_prefix(phone_number: &str) -> Option<&str> {
    if !is_phone_valid(phone_number) {
        return None;
    }

    for prefix in PREFIXES {
        if phone_number.starts_with(prefix) {
            return Some(&phone_number[prefix.len()..prefix.len() + 3]);
        }
    }

    None
}

#[cfg(test)]
mod test_phone_number {
    use super::*;

    #[test]
    fn check_phone_number_valid() {
        assert_eq!(is_phone_valid("9122221811"), true);
        assert_eq!(is_phone_valid("09122221811"), true);
        assert_eq!(is_phone_valid("+989122221811"), true);
        assert_eq!(is_phone_valid("12903908"), false);
        assert_eq!(is_phone_valid("901239812390812908"), false);
    }

    #[test]
    fn test_phone_number_normilizer() {
        // normalize to 0
        assert_eq!(
            phone_number_normalizer("+989373708555", "0"),
            Some("09373708555".to_string())
        );
        assert_eq!(
            phone_number_normalizer("989373708555", "0"),
            Some("09373708555".to_string())
        );
        assert_eq!(
            phone_number_normalizer("00989022002580", "0"),
            Some("09022002580".to_string())
        );
        assert_eq!(
            phone_number_normalizer("09122002580", "0"),
            Some("09122002580".to_string())
        );
        assert_eq!(
            phone_number_normalizer("9322002580", "0"),
            Some("09322002580".to_string())
        );

        // normalize to +98
        assert_eq!(
            phone_number_normalizer("09373708555", "+98"),
            Some("+989373708555".to_string())
        );
        assert_eq!(
            phone_number_normalizer("09022002580", "+98"),
            Some("+989022002580".to_string())
        );
        assert_eq!(
            phone_number_normalizer("09122002580", "+98"),
            Some("+989122002580".to_string())
        );
        assert_eq!(
            phone_number_normalizer("9322002580", "+98"),
            Some("+989322002580".to_string())
        );
        assert_eq!(
            phone_number_normalizer("00989022002580", "+98"),
            Some("+989022002580".to_string())
        );
    }

    #[test]
    fn test_phone_number_normilizer_invalid_phone() {
        assert_eq!(phone_number_normalizer("09132222", "+98"), None);
        assert_eq!(phone_number_normalizer("9191282819921", "0"), None);
    }

    #[test]
    fn test_operator_prefix() {
        assert_eq!(get_operator_prefix("+989373708555"), Some("937"));
        assert_eq!(get_operator_prefix("00989013708555"), Some("901"));
        assert_eq!(get_operator_prefix("00988013708555"), None);
    }
}
