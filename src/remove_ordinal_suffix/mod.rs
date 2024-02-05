///  Remove Ordinal suffix to numbers <br>
///  # Example:
///  --- input: چهل و سوم
/// ```
/// use rust_persian_tools::remove_ordinal_suffix::remove_ordinal_suffix;
/// assert_eq!(remove_ordinal_suffix("چهل و سوم"), "چهل و سه");
/// ```
///  --- output: چهل و سه <br>
pub fn remove_ordinal_suffix(word: impl AsRef<str>) -> String {
    let mut word: String = word.as_ref().to_string(); //allocate

    if word.ends_with("مین") {
        word = word[0..word.len() - ("مین".len())].to_string()
    } else if word.ends_with("اُم") {
        word = word[0..word.len() - ("اُم".len())].trim().to_string()
    } else if word.ends_with("ام") {
        word = word[0..word.len() - ("ام".len())].trim().to_string()
    } else if word.ends_with("سوم") {
        word = word[0..word.len() - ("سوم".len())].to_string();
        word += "سه";
    } else if word.ends_with('م') {
        word = word[0..word.len() - ("م".len())].to_string()
    }

    word
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
}
