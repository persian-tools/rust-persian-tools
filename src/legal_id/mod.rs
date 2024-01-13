use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifyIranianLegalIdError {
    #[error("invalid legal id")]
    NotValid,
}

pub fn verifyIranianLegalId(inp: impl AsRef<str>) -> Result<(), VerifyIranianLegalIdError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_name() {
        assert_eq!(1, 1);
    }
}
