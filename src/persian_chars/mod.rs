pub mod chars {
    pub static FA_ALPHABET: &str = "ابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی";
    pub static FA_NUMBER: &str = "۰۱۲۳۴۵۶۷۸۹";
    pub static FA_SHORT_VOWELS: &str = "َُِ";
    pub static FA_OTHERS: &str = "‌آاً";
    pub static FA_MIXED_WITH_ARABIC: &str = "ًٌٍَُِّْٰٔءك‌ةۀأإيـئؤ،";
    pub static FA_TEXT: &str = "ابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی۰۱۲۳۴۵۶۷۸۹َُِ‌آاً؟ ";
    pub static FA_COMPLEX_TEXT: &str = "ابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی۰۱۲۳۴۵۶۷۸۹َُِ‌آأًًٌٍَُِّْٰءك‌ةۀأإيـئؤ،؟ ";
}

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
    let fa_regex = if is_complex {
        chars::FA_COMPLEX_TEXT
    } else {
        chars::FA_TEXT
    };
    for c in str.chars() {
        if fa_regex.contains(c) {
            return true;
        }
    }
    false
}

/// isComplex in complex mode, we accept some specific characters which are common in Farsi text.<br>
/// Return true if the entered string does not include other-language characters.
pub fn is_persian<S>(str: S, is_complex: bool) -> bool
where
    S: Into<String>,
{
    let str: String = str.into();

    if str.is_empty() {
        return false;
    }
    let fa_regex = if is_complex {
        chars::FA_COMPLEX_TEXT
    } else {
        chars::FA_TEXT
    };
    for c in str.chars() {
        if !fa_regex.contains(c) {
            return false;
        }
    }
    true
}

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

    #[test]
    fn is_persian_test() {
        assert_eq!(is_persian("این یک متن فارسی است؟", false), true);
        assert_eq!(
            is_persian(
                "آیا سیستم میتواند گزینه های دیگری را به اشتباه به عنوان متن فارسی تشخیص دهد؟",
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

    #[test]
    fn test_name() {
        assert_eq!(to_persian_chars("علي"), "علی");

        // check if replace works to the end of line
        assert_eq!(to_persian_chars("علي علي"), "علی علی");
    }
}
