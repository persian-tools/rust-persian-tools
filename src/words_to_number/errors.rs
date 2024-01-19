use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum WordsToNumberError {
    #[error("There is a invalid unit in the input")]
    InvalidUnit,
    #[error("The input cannot be a empty string")]
    EmptyInput,
}
