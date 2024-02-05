//! Iranian National Number utils (`national-id` Cargo feature).
//!
//! #### Example
//! ```rust
//! use rust_persian_tools::national_id::{NationalIdError, verify_iranian_national_id};
//!
//! assert!(verify_iranian_national_id("11537027").is_ok());
//! assert!(verify_iranian_national_id("0076229645").is_ok());
//! assert!(verify_iranian_national_id("1583250689").is_ok());
//!
//! assert_eq!(
//!     verify_iranian_national_id("12345"),
//!     Err(NationalIdError::Length(5)),
//! );
//! assert_eq!(
//!     verify_iranian_national_id("9999999999"),
//!     Err(NationalIdError::Invalid)
//! );
//! ```
//!
//! #### [serde] Integration
//! ##### National Number
//! ```rust
//! use rust_persian_tools::national_id::serde::national_id_de;
//!
//! #[derive(Debug, PartialEq, serde::Deserialize)]
//! struct MyStruct {
//!     #[serde(deserialize_with = "national_id_de")]
//!     id: String,
//! }
//!
//! let json_str = "{\"id\": \"0076229645\"}";
//! let my_struct: MyStruct = serde_json::from_str(json_str).unwrap();
//! assert_eq!(my_struct, MyStruct{id: "0076229645".to_string()});
//!
//! let json_str_invalid = "{\"id\": \"ZeroOneTwo\"}";
//! assert!(serde_json::from_str::<MyStruct>(json_str_invalid).is_err());
//! assert_eq!(
//!     serde_json::from_str::<MyStruct>(json_str_invalid).err().unwrap().to_string(),
//!     "Could not convert National Number to numeric at line 1 column 19".to_string(),
//! );
//! ```
//! ##### Option\<National Number\>
//! ```rust
//! use rust_persian_tools::national_id::serde::national_id_option_de;
//!
//! #[derive(Debug, PartialEq, serde::Deserialize)]
//! struct MyStruct {
//!     #[serde(default, deserialize_with = "national_id_option_de")]
//!     id: Option<String>,
//! }
//!
//! let json_str = "{}";
//! let my_struct: MyStruct = serde_json::from_str(json_str).unwrap();
//! assert_eq!(my_struct, MyStruct{id: None});
//! let json_str = "{\"id\": \"0076229645\"}";
//! let my_struct: MyStruct = serde_json::from_str(json_str).unwrap();
//! assert_eq!(my_struct, MyStruct{id: Some("0076229645".to_string())});
//! ```

use std::num::ParseIntError;

#[cfg(feature = "serde")]
pub mod serde;

/// Possible errors during validation of Iranian National Number.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum NationalIdError {
    /// If input length is invalid.
    #[error("Invalid length {0} for National Number")]
    Length(usize),
    /// If input is not a [u64] number.
    #[error("Could not convert National Number to numeric")]
    NumericConvert { source: ParseIntError },
    /// Other checks.
    #[error("National Number is invalid")]
    Invalid,
}

/// Validation of Iranian National Number (code-e Melli).
///
/// ## Examples
/// ```rust
/// use rust_persian_tools::national_id::{NationalIdError, verify_iranian_national_id};
///
/// assert!(verify_iranian_national_id("68415941").is_ok());
/// assert!(verify_iranian_national_id("0200203241").is_ok());
/// assert!(verify_iranian_national_id("0067749828").is_ok());
///
/// assert_eq!(
///     verify_iranian_national_id("0"),
///     Err(NationalIdError::Length(1)),
/// );
/// assert_eq!(
///     verify_iranian_national_id("1230000000"),
///     Err(NationalIdError::Invalid)
/// );
/// ```
pub fn verify_iranian_national_id(code: impl AsRef<str>) -> Result<(), NationalIdError> {
    let code_str = code.as_ref();

    let length = code_str.len();
    if !((8..=10).contains(&length)) {
        return Err(NationalIdError::Length(length));
    }

    let code_u64 = code_str
        .parse::<u64>()
        .map_err(|source| NationalIdError::NumericConvert { source })?;

    if length == 10 && (code_u64 == 0 || are_digits_the_same(code_u64)) {
        return Err(NationalIdError::Invalid);
    }

    let code_str = &("00".to_owned() + code_str)[length + 2 - 10..];
    if code_str[3..9].parse::<u64>().unwrap() == 0 {
        return Err(NationalIdError::Invalid);
    }

    let mut sum = (0usize..9).fold(0, |sum, i| {
        sum + code_str[i..i + 1].parse::<usize>().unwrap() * (10 - i)
    });
    sum %= 11;
    let last_number = (code_u64 % 10) as usize;
    if (sum < 2 && last_number == sum) || (sum >= 2 && last_number == 11 - sum) {
        Ok(())
    } else {
        Err(NationalIdError::Invalid)
    }
}

#[inline]
fn are_digits_the_same(mut number: u64) -> bool {
    let last = number % 10;
    while number != 0 {
        let current = number % 10;
        number /= 10;
        if current != last {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod verify_iranian_national_id_tests {
    use super::*;

    #[test]
    fn check_falsy() {
        assert_eq!(
            verify_iranian_national_id(""),
            Err(NationalIdError::Length(0))
        );
        assert_eq!(
            verify_iranian_national_id("12345"),
            Err(NationalIdError::Length(5))
        );
        assert_eq!(
            verify_iranian_national_id("0"),
            Err(NationalIdError::Length(1))
        );
        assert_eq!(
            verify_iranian_national_id("000000"),
            Err(NationalIdError::Length(6))
        );
        assert_eq!(
            verify_iranian_national_id("12300000"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("123000000"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("1230000000"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("0000000000"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("4444444444"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("9999999999"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("0684159415"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("1111111111"),
            Err(NationalIdError::Invalid)
        );
        assert_eq!(
            verify_iranian_national_id("079041a904"),
            Err(NationalIdError::NumericConvert {
                source: "079041a904".parse::<u64>().err().unwrap()
            })
        ); // this is not in typescript version
    }

    #[test]
    fn check_truly() {
        assert_eq!(verify_iranian_national_id("11537027"), Ok(()));
        assert_eq!(verify_iranian_national_id("787833770"), Ok(()));
        assert_eq!(verify_iranian_national_id("1583250689"), Ok(()));
        assert_eq!(verify_iranian_national_id("0499370899"), Ok(()));
        assert_eq!(verify_iranian_national_id("0790419904"), Ok(()));
        assert_eq!(verify_iranian_national_id("0084575948"), Ok(()));
        assert_eq!(verify_iranian_national_id("0963695398"), Ok(()));
        assert_eq!(verify_iranian_national_id("0684159414"), Ok(()));
        assert_eq!(verify_iranian_national_id("0067749828"), Ok(()));
        assert_eq!(verify_iranian_national_id("0650451252"), Ok(()));
        assert_eq!(verify_iranian_national_id("4032152314"), Ok(()));
        assert_eq!(verify_iranian_national_id("0076229645"), Ok(()));
        assert_eq!(verify_iranian_national_id("4271467685"), Ok(()));
        assert_eq!(verify_iranian_national_id("0200203241"), Ok(()));
        assert_eq!(verify_iranian_national_id("068415941"), Ok(()));
        assert_eq!(verify_iranian_national_id("68415941"), Ok(()));
        assert_eq!(verify_iranian_national_id("787833770"), Ok(()));
    }
}
