use std::num::ParseIntError;

use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ShebaValidationError {
    #[error("The input is empty.")]
    Empty,

    #[error("The input does not start with 'IR'.")]
    NotStartedWithIR,

    #[error("There is an invalid digit in the input.")]
    InvalidDigit,

    #[error("Input length is invalid: {0:?}")]
    InvalidLength(usize),

    #[error("Input checksum is invalid.")]
    InvalidChecksum,

    #[error("Bank information not found.")]
    BankNotFound,

    #[error("You should not see this! Please create an issue on our GitHub repository.")]
    InternalError,
}

impl From<ParseIntError> for ShebaValidationError {
    fn from(_: ParseIntError) -> Self {
        ShebaValidationError::InvalidDigit
    }
}
