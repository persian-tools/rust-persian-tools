/// Remove all commas in string\
/// 3,000,000 -> 3000000\
/// Example:
/// ```rust
/// // function:
/// use rust_persian_tools::commas::remove_commas::remove_commas;
/// assert_eq!(remove_commas("30,000,000"), "30000000".to_string());
///
/// // method:
/// use crate::rust_persian_tools::commas::remove_commas::RemoveCommas;
/// assert_eq!("30,000,000".remove_commas(), "30000000".to_string());
/// ```
pub fn remove_commas(str: impl Into<String>) -> String {
    let str: String = str.into();
    str.replace(',', "")
}

pub trait RemoveCommas {
    fn remove_commas(&self) -> String;
}

impl RemoveCommas for String {
    fn remove_commas(&self) -> String {
        remove_commas(self)
    }
}

impl RemoveCommas for str {
    fn remove_commas(&self) -> String {
        remove_commas(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_commas_test() {
        assert_eq!(remove_commas("30,000,000"), "30000000".to_string());
    }
}
