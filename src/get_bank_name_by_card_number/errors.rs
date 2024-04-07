use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Clone, PartialEq, Eq, Debug, Hash)]
pub enum BankNameByCardNumberError {
    #[error("Card number \"{0:?}\" with a length of {1:?} is too short; minimum length is 6.")]
    TooShort(String, usize),

    #[error("Card number {0:?} does not match any bank.")]
    NotFound(String),
}
