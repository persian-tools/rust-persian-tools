///  Remove Ordinal suffix to numbers <br>
///  # Example:
///  --- input: چهل و سوم
///  --- output: چهل و سه <br>
/// ```
/// use rust_persian_tools::remove_ordinal_suffix::remove_ordinal_suffix;
/// assert_eq!(remove_ordinal_suffix("چهل و سوم"), "چهل و سه");
/// ```
pub fn remove_ordinal_suffix(word: impl AsRef<str>) -> String {
    let mut word: String = word.as_ref().to_string(); //allocate
    remove_ordinal_suffix_mut(&mut word);
    word
}

pub trait RemoveOrdinalSuffix {
    fn remove_ordinal_suffix(&self) -> String;
}

impl RemoveOrdinalSuffix for String {
    fn remove_ordinal_suffix(&self) -> String {
        remove_ordinal_suffix(self)
    }
}
impl RemoveOrdinalSuffix for str {
    fn remove_ordinal_suffix(&self) -> String {
        remove_ordinal_suffix(self)
    }
}
use std::borrow::Cow;
impl RemoveOrdinalSuffix for Cow<'_, str> {
    fn remove_ordinal_suffix(&self) -> String {
        remove_ordinal_suffix(self)
    }
}

/// Remove Ordinal Suffix of a number in-place without any allocation
///  Remove Ordinal suffix to numbers <br>
///  # Example:
///  --- input: چهل و سوم
///  --- edited input: چهل و سه <br>
/// ```
/// use rust_persian_tools::remove_ordinal_suffix::remove_ordinal_suffix_mut;
/// let mut word = String::from("چهل و سوم");
/// remove_ordinal_suffix_mut(&mut word);
/// assert_eq!(word, "چهل و سه");
/// ```
pub fn remove_ordinal_suffix_mut(word: &mut String) {
    if word.ends_with("مین") {
        *word = word[0..word.len() - ("مین".len())].to_string()
    } else if word.ends_with("اُم") {
        *word = word[0..word.len() - ("اُم".len())].trim().to_string()
    } else if word.ends_with("ام") {
        *word = word[0..word.len() - ("ام".len())].trim().to_string()
    } else if word.ends_with("سوم") {
        *word = word[0..word.len() - ("سوم".len())].to_string();
        *word += "سه";
    } else if word.ends_with('م') {
        *word = word[0..word.len() - ("م".len())].to_string()
    }
}

pub trait RemoveOrdinalSuffixMut {
    fn remove_ordinal_suffix_mut(&mut self);
}

impl RemoveOrdinalSuffixMut for String {
    fn remove_ordinal_suffix_mut(&mut self) {
        remove_ordinal_suffix_mut(self)
    }
}

impl RemoveOrdinalSuffixMut for Cow<'_, str> {
    fn remove_ordinal_suffix_mut(&mut self) {
        remove_ordinal_suffix_mut(self.to_mut())
    }
}

#[cfg(test)]
mod remove_ordinal_suffix_tests {
    use super::*;

    #[test]
    fn remove_test() {
        assert_eq!(remove_ordinal_suffix("چهل و سوم"), "چهل و سه");
        assert_eq!(remove_ordinal_suffix("چهل و پنجم"), "چهل و پنج");
        assert_eq!(remove_ordinal_suffix("سی اُم"), "سی");
    }

    #[test]
    fn remove_mut_test() {
        let mut word = String::from("چهل و سوم");
        remove_ordinal_suffix_mut(&mut word);
        assert_eq!(word, "چهل و سه");
    }
}
