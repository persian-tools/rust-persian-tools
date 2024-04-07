use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum PlateNumberError {
    #[error("Plate numbers must be 7 or 8 digits long.")]
    InvalidPlateDigitLength,
    #[error("Invalid plate character length. Motorcycles should have 7 characters, and cars should have 8.")]
    InvalidPlateCharacterLength,
    #[error("Invalid plate character: {0}")]
    InvalidPlateCharacter(String),
    #[error("Invalid motorcycle province code: {0:?}")]
    MotorcycleProvinceNotFound(String),
    #[error("Invalid car province code: {0:?}")]
    CarProvinceNotFound(String),
}
