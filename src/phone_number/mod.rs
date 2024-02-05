pub mod operators;

use thiserror::Error;

pub static PREFIXES: [&str; 4] = ["+98", "98", "0098", "0"];

#[derive(Error, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PhoneNumberError {
    #[error("This prefix is not a valid phone number (prefix : `{0}`)")]
    InvalidPrefix(String),
    #[error("The phone number format is invalid")]
    InvalidFormat,
    #[error("Unexpected error happened !")]
    Unknown,
}

/// This is a simple function that checks if a phone number valid or not
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::is_phone_valid;
///
/// assert!(is_phone_valid("+989122221811").is_ok());
/// assert!(is_phone_valid("12903908").is_err());
/// ```
pub fn is_phone_valid(phone_number: impl AsRef<str>) -> Result<(), PhoneNumberError> {
    let phone_number = phone_number.as_ref();

    let prefix = get_phone_prefix(phone_number).unwrap_or("");

    let phone_number_without_prefix = &phone_number[prefix.len()..];

    if phone_number_without_prefix.len() == 10 && phone_number_without_prefix.starts_with('9') {
        return Ok(());
    }

    Err(PhoneNumberError::InvalidFormat)
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
/// assert_eq!(get_phone_prefix("00989122221811").unwrap(),"0098");
/// assert_eq!(get_phone_prefix("09122221811").unwrap(),"0");
/// assert!(get_phone_prefix("29122221811").is_err());
/// ```
pub fn get_phone_prefix(phone_number: impl AsRef<str>) -> Result<&'static str, PhoneNumberError> {
    let phone_number = phone_number.as_ref();

    let prefix = PREFIXES
        .into_iter()
        .find(|&prefix| phone_number.starts_with(prefix));

    match prefix {
        Some(pre) => Ok(pre),
        None => Err(PhoneNumberError::InvalidFormat),
    }
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
/// assert_eq!(phone_number_normalizer("+989373708555", "0").unwrap(),"09373708555".to_string());
/// assert!(phone_number_normalizer("09132222", "+98").is_err());
/// ```
pub fn phone_number_normalizer(
    phone_number: impl AsRef<str>,
    new_prefix: impl AsRef<str>,
) -> Result<String, PhoneNumberError> {
    let phone_number = phone_number.as_ref();
    let new_prefix = new_prefix.as_ref();

    is_phone_valid(phone_number)?;

    if let Ok(prefix) = get_phone_prefix(phone_number) {
        let (_, splited) = phone_number.split_at(prefix.len());
        return Ok(format!("{new_prefix}{splited}"));
    }

    Ok(format!("{new_prefix}{phone_number}"))
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
/// assert_eq!(get_operator_prefix("00989013708555").unwrap() , "901");
/// assert!(get_operator_prefix("00988013708555").is_err());
/// ```
pub fn get_operator_prefix(phone_number: &str) -> Result<&str, PhoneNumberError> {
    is_phone_valid(phone_number)?;

    for prefix in PREFIXES {
        if phone_number.starts_with(prefix) {
            return Ok(&phone_number[prefix.len()..prefix.len() + 3]);
        }
    }

    Err(PhoneNumberError::InvalidFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_phone_number_valid() {
        assert!(is_phone_valid("9122221811").is_ok());
        assert!(is_phone_valid("09122221811").is_ok());
        assert!(is_phone_valid("+989122221811").is_ok());
        assert!(is_phone_valid("12903908").is_err());
        assert!(is_phone_valid("901239812390812908").is_err());
    }

    #[test]
    fn test_phone_number_normilizer() {
        // normalize to 0

        assert_eq!(
            phone_number_normalizer("+989373708555", "0").unwrap(),
            "09373708555".to_string()
        );

        assert_eq!(
            phone_number_normalizer("989373708555", "0").unwrap(),
            "09373708555".to_string()
        );

        assert_eq!(
            phone_number_normalizer("00989022002580", "0").unwrap(),
            "09022002580".to_string()
        );
        assert_eq!(
            phone_number_normalizer("09122002580", "0").unwrap(),
            "09122002580".to_string()
        );
        assert_eq!(
            phone_number_normalizer("9322002580", "0").unwrap(),
            "09322002580".to_string()
        );

        // normalize to +98
        assert_eq!(
            phone_number_normalizer("09373708555", "+98").unwrap(),
            "+989373708555".to_string()
        );
        assert_eq!(
            phone_number_normalizer("09022002580", "+98").unwrap(),
            "+989022002580".to_string()
        );
        assert_eq!(
            phone_number_normalizer("09122002580", "+98").unwrap(),
            "+989122002580".to_string()
        );
        assert_eq!(
            phone_number_normalizer("9322002580", "+98").unwrap(),
            "+989322002580".to_string()
        );
        assert_eq!(
            phone_number_normalizer("00989022002580", "+98").unwrap(),
            "+989022002580".to_string()
        );
    }

    #[test]
    fn test_phone_number_normilizer_invalid_phone() {
        assert!(phone_number_normalizer("09132222", "+98").is_err());
        assert!(phone_number_normalizer("9191282819921", "0").is_err());
    }

    #[test]
    fn test_operator_prefix() {
        assert_eq!(get_operator_prefix("+989373708555").unwrap(), "937");
        assert_eq!(get_operator_prefix("00989013708555").unwrap(), "901");
        assert!(get_operator_prefix("00988013708555").is_err());
    }
}
