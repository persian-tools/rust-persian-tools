use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Clone, Debug, Hash, PartialEq, Eq)]
pub enum NumberToWordsError {
    #[error("The input is empty.")]
    EmptyString,

    #[error("There is an invalid integer in the input: \"{0}\"")]
    InvalidInteger(String),

    #[error("Overflow occurred while processing the integer: \"{0}\"")]
    Overflow(String),

    #[error("Unknown error occurred: \"{0}\"")]
    Unknown(String),

    #[error("You should not see this! Please create an issue on our GitHub repository.")]
    Internal,
}
