use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PlaceByNationalIdError {
    #[error("national id is too short len:{0}")]
    TooShortNationalId(usize),

    #[error("place not found")]
    NotFound,
}
