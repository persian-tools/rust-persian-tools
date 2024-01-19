pub mod errors;

use std::borrow::Cow;

use errors::VerifyLegalIdError;

type E = VerifyLegalIdError;

/// Check Legal-id validation AKA شناسه حقوقی (shenasi hoghooghi)\
/// Example:
/// ```
/// use rust_persian_tools::legal_id::verify_iranian_legal_id;
/// use rust_persian_tools::legal_id::errors::VerifyLegalIdError;
/// assert_eq!(verify_iranian_legal_id("10380284790"), Ok(()));
/// assert_eq!(
///     verify_iranian_legal_id("123000000"),
///     Err(VerifyLegalIdError::InvalidLength)
/// );
/// assert_eq!(
///     verify_iranian_legal_id("11111111111"),
///     Err(VerifyLegalIdError::InvalidChecksum)
/// );
/// ```
pub fn verify_iranian_legal_id(legal_id: impl AsRef<str>) -> Result<(), E> {
    let legal_id = legal_id.as_ref();

    // length
    if legal_id.len() != 11 {
        return Err(E::InvalidLength);
    }

    // parse digits to int
    let mut digits: Vec<u32> = Vec::with_capacity(11);
    for c in legal_id.chars() {
        digits.push(c.to_digit(10).ok_or(E::InternalError)?);
    }

    // all zero (error)
    if digits.iter().all(|x| x == &0) {
        return Err(E::Invalid);
    }

    // 3..9 zero (error)
    if digits.iter().skip(3).take(6).all(|c| c == &0) {
        return Err(E::Invalid);
    }

    // checksum
    let extracted_checksum = digits.get(10).ok_or(E::InternalError)?;
    let calculated_checksum = checksum(&digits)?;
    if *extracted_checksum == calculated_checksum {
        Ok(())
    } else {
        Err(E::InvalidChecksum)
    }
}

/// 10th digit of legal id is checksum\
/// this function calculate checksum from first 9 digits\
/// returns calculated checksum\
fn checksum(digits: &[u32]) -> Result<u32, E> {
    let d = (digits.get(9).ok_or(E::InternalError)?) + 2;

    const Z: [u32; 5] = [29, 27, 23, 19, 17];
    let mut sum = 0;
    for i in 0..10 {
        sum += (d + digits.get(i).ok_or(E::InternalError)?) * Z[i % 5];
    }
    sum %= 11;
    Ok(if sum == 10 { 0 } else { sum })
}

trait VerifyLegalId {
    fn verify_iranian_legal_id(&self) -> Result<(), E>;
}

impl VerifyLegalId for str {
    fn verify_iranian_legal_id(&self) -> Result<(), E> {
        verify_iranian_legal_id(self)
    }
}

impl VerifyLegalId for String {
    fn verify_iranian_legal_id(&self) -> Result<(), E> {
        verify_iranian_legal_id(self)
    }
}

impl VerifyLegalId for Cow<'_, str> {
    fn verify_iranian_legal_id(&self) -> Result<(), E> {
        verify_iranian_legal_id(self)
    }
}

impl VerifyLegalId for Cow<'_, String> {
    fn verify_iranian_legal_id(&self) -> Result<(), E> {
        match self {
            Cow::Borrowed(b) => verify_iranian_legal_id(b),
            Cow::Owned(o) => verify_iranian_legal_id(o),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_iranian_legal_id_test() {
        // valid
        assert_eq!(verify_iranian_legal_id("10380284790"), Ok(()));

        // not valid
        assert_eq!(
            verify_iranian_legal_id("123000000"),
            Err(VerifyLegalIdError::InvalidLength)
        );
        assert_eq!(
            verify_iranian_legal_id("11111111111"),
            Err(VerifyLegalIdError::InvalidChecksum)
        );
        assert_eq!(
            verify_iranian_legal_id("1111111111"),
            Err(VerifyLegalIdError::InvalidLength)
        );
        assert_eq!(
            verify_iranian_legal_id("10380284792"),
            Err(VerifyLegalIdError::InvalidChecksum)
        );
        assert_eq!(
            verify_iranian_legal_id("10380285692"),
            Err(VerifyLegalIdError::InvalidChecksum)
        );
        assert_eq!(
            verify_iranian_legal_id("09748208301"),
            Err(VerifyLegalIdError::InvalidChecksum)
        );
    }
}
