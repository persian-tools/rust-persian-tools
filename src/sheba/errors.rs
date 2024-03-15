use std::num::ParseIntError;

use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ShebaValidationError {
    #[error("empty")]
    Empty,

    #[error("not started with IR")]
    NotStartedWithIR,

    #[error("invalid digit")]
    InvalidDigit,

    #[error("invalid length: {0:?}")]
    InvalidLength(usize),

    #[error("invalid checksum")]
    InvalidChecksum,

    #[error("bank not found")]
    BankNotFound,

    #[error("you should not see this! please make a issue on our github")]
    InternalError,
}

impl From<ParseIntError> for ShebaValidationError {
    fn from(_: ParseIntError) -> Self {
        ShebaValidationError::InvalidDigit
    }
}
