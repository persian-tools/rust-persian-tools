///  Remove Ordinal suffix to numbers <br>
///  Example: <br>
///  --- input: چهل و سوم
///  --- output: چهل و سه <br>
pub fn remove_ordinal_suffix<S>(word: S) -> String
where
    S: Into<String>,
{
    let mut word: String = word.into();

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
