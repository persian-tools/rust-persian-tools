/// Description: Replaces all instances of ي and ك withی and ک,
/// respectively. It should not make any ch anges to Arabic text
/// surrounded by appropriate templates.
pub fn to_persian_chars<S>(inp: S) -> String
where
    S: Into<String>,
{
    let inp: String = inp.into();

    inp.replace('ي', "ی").replace('ك', "ک").replace('ى', "ی")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(to_persian_chars("علي"), "علی");

        // check if replace works to the end of line
        assert_eq!(to_persian_chars("علي علي"), "علی علی");
    }
}
