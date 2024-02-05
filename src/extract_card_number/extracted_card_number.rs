#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct ExtractedCardNumber {
    base: String,
    pure: String,
}

impl ExtractedCardNumber {
    pub fn new(base: impl Into<String>, pure: impl Into<String>) -> Self {
        ExtractedCardNumber {
            base: base.into(),
            pure: pure.into(),
        }
    }

    pub fn get_base(&self) -> &str {
        &self.base
    }

    pub fn get_pure(&self) -> &str {
        &self.base
    }
}
