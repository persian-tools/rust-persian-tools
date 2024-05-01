use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PlaceByNationalIdError {
    #[error("National ID is too short. Length: {0}")]
    TooShortNationalId(usize),

    #[error("Place not found for the given National ID.")]
    NotFound,
}
