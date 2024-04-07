use std::num::ParseIntError;

use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum VerifyCardNumberError {
    #[error("There is an invalid digit in the input.")]
    InvalidDigit,

    #[error("Input length is invalid.")]
    InvalidLength,

    #[error("Input format is not a valid card number.")]
    InvalidCardNumber,
}

impl From<ParseIntError> for VerifyCardNumberError {
    fn from(_: ParseIntError) -> Self {
        VerifyCardNumberError::InvalidDigit
    }
}
