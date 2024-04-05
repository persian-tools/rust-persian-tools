mod bank_codes;
pub mod errors;
pub use errors::BankNameByCardNumberError;

use bank_codes::get_bank_code;

/// gets card number as input and detects bank name.\
/// 6037701689095443 -> "بانک کشاورزی"
/// note: card number can be incompelete 603770 -> "بانک کشاورزی"
/// # Example
/// ```
/// # use rust_persian_tools::get_bank_name_by_card_number::*;
/// assert_eq!(
///     get_bank_name_by_card_number("6037701689095443"),
///     Ok("بانک کشاورزی")
/// );
/// ```
pub fn get_bank_name_by_card_number(
    digits: impl AsRef<str>,
) -> Result<&'static str, BankNameByCardNumberError> {
    let digits = digits.as_ref();

    if digits.len() >= 6 {
        // slicing "&digits[0..6]" is safe because we are cheking length
        if let Some(x) = get_bank_code(&digits[0..6]) {
            Ok(x)
        } else {
            Err(BankNameByCardNumberError::NotFound(digits.to_string()))
        }
    } else {
        Err(BankNameByCardNumberError::TooShort(
            digits.to_string(),
            digits.len(),
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            get_bank_name_by_card_number("6037701689095443"),
            Ok("بانک کشاورزی")
        );
        assert_eq!(
            get_bank_name_by_card_number("6219861034529007"),
            Ok("بانک سامان")
        );
        assert_eq!(
            get_bank_name_by_card_number("6219861034529007"),
            Ok("بانک سامان")
        );

        assert_eq!(get_bank_name_by_card_number("610433"), Ok("بانک ملت"));
        assert_eq!(
            get_bank_name_by_card_number("50222919"),
            Ok("بانک پاسارگاد")
        );

        assert_eq!(
            get_bank_name_by_card_number("50222"),
            Err(BankNameByCardNumberError::TooShort("50222".to_string(), 5))
        );
        assert_eq!(
            get_bank_name_by_card_number("9999991034529007"),
            Err(BankNameByCardNumberError::NotFound(
                "9999991034529007".to_string()
            ))
        );
    }
}
