use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VerifyLegalIdError {
    #[error("Invalid length. Legal ID should be exactly 11 characters.")]
    InvalidLength,

    #[error("There is an invalid digit in the input.")]
    InvalidDigit,

    #[error("Legal ID is invalid.")]
    Invalid,

    #[error("Input checksum is invalid.")]
    InvalidChecksum,

    #[error("You should not see this! Please create an issue on our GitHub repository.")]
    InternalError,
}
