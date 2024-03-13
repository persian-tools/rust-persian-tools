use std::num::ParseIntError;

use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum VerifyCardNumberError {
    #[error("invalid digit")]
    InvalidDigit,

    #[error("invalid length")]
    InvalidLength,

    #[error("invalid length")]
    InvalidCardNumber,
}

impl From<ParseIntError> for VerifyCardNumberError {
    fn from(_: ParseIntError) -> Self {
        VerifyCardNumberError::InvalidDigit
    }
}
