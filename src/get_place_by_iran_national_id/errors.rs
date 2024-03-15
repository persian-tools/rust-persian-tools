use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Error, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PlaceByNationalIdError {
    #[error("national id is too short len:{0}")]
    TooShortNationalId(usize),

    #[error("place not found")]
    NotFound,
}
