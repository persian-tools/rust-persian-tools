use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum PlateNumberError {
    #[error("plate numbers must be 7 or 8 digits long")]
    InvalidPlateDigitLength,
    #[error("invalid plate charchter length (7 for motorcucles, 8 for cars)")]
    InvalidPlateCharacterLength,
    #[error("invalid plate charchter {0:?}")]
    InvalidPlateCharacter(String),
    #[error("invalid motorcycle province code {0:?}")]
    MotorcycleProvinceNotFound(String),
    #[error("invalid car province code {0:?}")]
    CarProvinceNotFound(String),
}
