use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Clone, Debug, Hash, PartialEq, Eq)]
pub enum NumberToWordsError {
    #[error("number_to_words_str input is empty")]
    EmptyString,

    #[error("number_to_words_str invalid integer -> \"{0}\"")]
    InvalidInteger(String),

    #[error("number_to_words_str integer overflow -> \"{0}\"")]
    Overflow(String),

    #[error("number_to_words_str unknown -> \"{0}\"")]
    Unknown(String),

    #[error("number_to_words_str you should not see this error, please make an issue on github")]
    Internal,
}
