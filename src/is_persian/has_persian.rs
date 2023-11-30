use super::farsi_chars::{FA_COMPLEX_TEXT, FA_TEXT};

/// isComplex in complex mode, we accept some specific characters which are common in Farsi text.<br>
/// Return true if the entered string includes persian characters
pub fn has_persian<S>(str: S, is_complex: bool) -> bool
where
    S: Into<String>,
{
    let str: String = str.into();
    let str = str.replace(' ', "");
    if str.is_empty() {
        return false;
    }
    let fa_regex = if is_complex { FA_COMPLEX_TEXT } else { FA_TEXT };
    for c in str.chars() {
        if fa_regex.contains(c) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_persian_test() {
        assert_eq!(has_persian("این یک متن فارسی است؟", false), true);
        assert_eq!(has_persian("هل هذا نص فارسي؟", false), true);
        assert_eq!(
            has_persian(
                "آیا سیستم میتواند گزینه های دیگری را به اشتباه به عنوان متن فارسی تشخیص دهد؟",
                false
            ),
            true
        );
        assert_eq!(has_persian("This text includes فارسی", false), true);
        assert_eq!(has_persian("Это персидский س текст?", false), true);
        assert_eq!(
            has_persian(
                "أكد رئيس اللجنة العسكرية الممثلة لحكومة الوفاق أراضي البلاد.",
                false
            ),
            true
        );

        assert_eq!(has_persian("Lorem Ipsum Test", false), false);
        assert_eq!(
            has_persian("これはペルシア語のテキストですか", false),
            false
        );
        assert_eq!(has_persian("Это персидский текст?", false), false);
        assert_eq!(has_persian("这是波斯文字吗?", false), false);
        assert_eq!(has_persian("", false), false);
    }
}
