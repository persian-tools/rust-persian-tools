use super::farsi_chars::{FA_COMPLEX_TEXT, FA_TEXT};

/// isComplex in complex mode, we accept some specific characters which are common in Farsi text.<br>
/// Return true if the entered string does not include other-language characters.
pub fn is_persian<S>(str: S, is_complex: bool) -> bool
where
    S: Into<String>,
{
    // let str = str.into();
    let trim_pattern = regex::Regex::new(r#"[\"'\-\+\(\)\!\?\s.\\،\\؛]"#).unwrap();
    let str: String = trim_pattern.replace_all(&str.into(), "").into_owned();

    if str.is_empty() {
        return false;
    }
    let fa_regex = if is_complex { FA_COMPLEX_TEXT } else { FA_TEXT };
    for c in str.chars() {
        if !fa_regex.contains(c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::to_persian_chars::to_persian_chars;

    use super::*;

    #[test]
    fn is_persian_test() {
        assert_eq!(is_persian("این، یک متن فارسی است؟", false), true);
        assert_eq!(is_persian("این؛ یک متن فارسی است.", false), true);
        assert_eq!(
            is_persian(
                "!آیا سیستم میتواند گزینه های دیگری را به اشتباه به عنوان متن فارسی تشخیص دهد؟",
                false
            ),
            true
        );

        assert_eq!(is_persian("Lorem Ipsum Test", false), false);
        assert_eq!(is_persian("これはペルシア語のテキストですか", false), false);
        assert_eq!(is_persian("Это персидский текст?", false), false);
        assert_eq!(is_persian("这是波斯文字吗?", false), false);
        assert_eq!(is_persian("هل هذا نص فارسي؟", false), false);
        assert_eq!(
			is_persian("أكد رئيس اللجنة العسكرية الممثلة لحكومة الوفاق الوطني في ليبيا أحمد علي أبو شحمة، أن اللجنة لا تستطيع تنفيذ خطتها لإخراج العناصر الأجنبية من أراضي البلاد.",false),
            false
		);
        assert_eq!(is_persian("", false), false);
        assert_eq!(is_persian(to_persian_chars("مهدی"), false), true);
        assert_eq!(is_persian(to_persian_chars("شاه"), false), true);
    }
}
