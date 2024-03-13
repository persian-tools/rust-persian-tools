use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum WordsToNumberError {
    #[error("There is a invalid unit in the input")]
    InvalidUnit,
    #[error("The input cannot be a empty string")]
    EmptyInput,
}
