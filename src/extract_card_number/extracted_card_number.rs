#[derive(PartialEq, Eq, Debug)]
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
}
