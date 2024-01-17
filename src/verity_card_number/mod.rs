pub mod errors;
use errors::VerifyCardNumberError;

///  bank card number validation
/// Example:
/// ```
/// # use rust_persian_tools::verity_card_number::verify_card_number;
/// # use rust_persian_tools::verity_card_number::errors::VerifyCardNumberError;
/// assert_eq!(verify_card_number("6037701689095443"), Ok(()));
/// assert_eq!(
///    verify_card_number("621986103452900"),
///    Err(VerifyCardNumberError::InvalidLength)
/// );
/// ```
/// safty: sum max posible value is 144 which is smaller than u8:MAX = 2^8 = 255
pub fn verify_card_number(digits: impl AsRef<str>) -> Result<(), VerifyCardNumberError> {
    let digits = digits.as_ref();

    if digits.len() != 16 {
        return Err(VerifyCardNumberError::InvalidLength);
    }

    let mut sum = 0u8;
    for (i, digit) in digits.chars().enumerate() {
        let radix = if i % 2 == 0 { 2 } else { 1 };

        let sub_digit = (digit).to_string().parse::<u8>()? * radix;
        sum += if sub_digit > 9 {
            sub_digit - 9
        } else {
            sub_digit
        };
    }

    if sum % 10 == 0 {
        Ok(())
    } else {
        Err(VerifyCardNumberError::InvalidCardNumber)
    }
}

#[cfg(test)]
mod add_ordinal_suffix_tests {
    use super::*;

    #[test]
    fn card_number_validation() {
        assert_eq!(verify_card_number("6037701689095443"), Ok(()));
        assert_eq!(verify_card_number("6219861034529007"), Ok(()));
    }

    #[test]
    fn card_number_validation_falsy() {
        assert_eq!(
            verify_card_number("62198610345290078"), // len: 17
            Err(VerifyCardNumberError::InvalidLength)
        );
        assert_eq!(
            verify_card_number("621986103452900"), // len: 15
            Err(VerifyCardNumberError::InvalidLength)
        );
        assert_eq!(
            verify_card_number("0"), // len: 1
            Err(VerifyCardNumberError::InvalidLength)
        );
        assert_eq!(
            verify_card_number("6219861A34529008"), // A
            Err(VerifyCardNumberError::InvalidDigit)
        );
        assert_eq!(
            verify_card_number("6219861034529008"), // sumcheck
            Err(VerifyCardNumberError::InvalidCardNumber)
        );
    }
}
