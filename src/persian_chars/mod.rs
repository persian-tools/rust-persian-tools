use std::borrow::Cow;

pub mod chars {
    pub static FA_TEXT: &str = "ابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی۰۱۲۳۴۵۶۷۸۹َُِ‌آاً؟ ";
    pub static FA_COMPLEX_TEXT: &str = "ابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی۰۱۲۳۴۵۶۷۸۹َُِ‌آأًًٌٍَُِّْٰءك‌ةۀأإيـئؤ،؟ ";
}

/// isComplex in complex mode, we accept some specific characters which are common in Farsi text.<br>
/// Return true if the entered string includes persian characters
pub fn has_persian(input: impl AsRef<str>, is_complex: bool) -> bool {
    let fa_chars = if is_complex {
        chars::FA_COMPLEX_TEXT
    } else {
        chars::FA_TEXT
    };
    input
        .as_ref()
        .chars()
        .any(|char| char != ' ' && fa_chars.contains(char))
}

/// isComplex in complex mode, we accept some specific characters which are common in Farsi text.<br>
/// Return true if the entered string does not include other-language characters.
pub fn is_persian(input: impl AsRef<str>, is_complex: bool) -> bool {
    let fa_chars = if is_complex {
        chars::FA_COMPLEX_TEXT
    } else {
        chars::FA_TEXT
    };
    !input.as_ref().is_empty() && input.as_ref().chars().all(|char| fa_chars.contains(char))
}

/// Description: Replaces all instances of ي and ك withی and ک,
/// respectively. It should not make any ch anges to Arabic text
/// surrounded by appropriate templates.
pub fn to_persian_chars(input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .replace('ي', "ی")
        .replace('ك', "ک")
        .replace('ى', "ی")
}

pub fn to_persian_chars_mut<I>(mut input: I)
where
    I: AsMut<str> + AsRef<str>,
{
    input
        .as_ref()
        .match_indices('ي')
        .chain(input.as_ref().match_indices('ك'))
        .chain(input.as_ref().match_indices('ى'))
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|index|
            // SAFETY:
            // We do not change the length of `&mut str`
            // We replace two bytes values only.
            unsafe {
            // Bytes for 'ي': [0xD9, 0x8A]
            // Bytes for 'ى': [0xD9, 0x89]
            // Bytes for 'ك': [0xD9, 0x83]
            // First byte for all of them is 0xD9, So we only need to check second byte.
            let next_byte = input.as_mut().as_bytes_mut().get_unchecked_mut(index + 1);
            match next_byte {
                // Characters 'ي' and 'ى' will be converted to 'ی' which is [0xDB, 0x8C]
                0x8A | 0x89 => {
                    *next_byte = 0x8C;
                    *input.as_mut().as_bytes_mut().get_unchecked_mut(index) = 0xDB;
                }
                // Character 'ك' will be converted to 'ی' which is [0xDA, 0xA9]
                0x83 => {
                    *next_byte = 0xA9;
                    *input.as_mut().as_bytes_mut().get_unchecked_mut(index) = 0xDA;
                }
                other => {
                    unreachable!("Second byte: {other}")
                }
            }
        });
}

pub trait ToPersianChars {
    fn to_persian_chars(&self) -> String;
}

pub trait ToPersianCharsMut {
    fn to_persian_chars_mut(&mut self);
}

pub trait HasPersian {
    fn has_persian(&self, is_complex: bool) -> bool;
}

pub trait IsPersian {
    fn is_persian(&self, is_complex: bool) -> bool;
}

impl ToPersianChars for str {
    fn to_persian_chars(&self) -> String {
        to_persian_chars(self)
    }
}

impl ToPersianChars for String {
    fn to_persian_chars(&self) -> String {
        to_persian_chars(self)
    }
}

impl ToPersianChars for Cow<'_, str> {
    fn to_persian_chars(&self) -> String {
        to_persian_chars(self)
    }
}

impl ToPersianCharsMut for String {
    fn to_persian_chars_mut(&mut self) {
        to_persian_chars_mut(self)
    }
}

impl ToPersianCharsMut for Cow<'_, str> {
    fn to_persian_chars_mut(&mut self) {
        to_persian_chars_mut(self.to_mut())
    }
}

impl HasPersian for str {
    fn has_persian(&self, is_complex: bool) -> bool {
        has_persian(self, is_complex)
    }
}

impl HasPersian for String {
    fn has_persian(&self, is_complex: bool) -> bool {
        has_persian(self, is_complex)
    }
}

impl HasPersian for Cow<'_, str> {
    fn has_persian(&self, is_complex: bool) -> bool {
        has_persian(self, is_complex)
    }
}

impl IsPersian for str {
    fn is_persian(&self, is_complex: bool) -> bool {
        is_persian(self, is_complex)
    }
}

impl IsPersian for String {
    fn is_persian(&self, is_complex: bool) -> bool {
        is_persian(self, is_complex)
    }
}

impl IsPersian for Cow<'_, str> {
    fn is_persian(&self, is_complex: bool) -> bool {
        is_persian(self, is_complex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_persian_test() {
        assert_eq!("این یک متن فارسی است؟".has_persian(false), true);
        assert_eq!("هل هذا نص فارسي؟".has_persian(false), true);
        assert_eq!(
            "آیا سیستم میتواند گزینه های دیگری را به اشتباه به عنوان متن فارسی تشخیص دهد؟"
                .has_persian(false),
            true
        );
        assert_eq!("This text includes فارسی".has_persian(false), true);
        assert_eq!("Это персидский س текст?".has_persian(false), true);
        assert_eq!(
            "أكد رئيس اللجنة العسكرية الممثلة لحكومة الوفاق أراضي البلاد.".has_persian(false),
            true
        );

        assert_eq!("Lorem Ipsum Test".has_persian(false), false);
        assert_eq!("これはペルシア語のテキストですか".has_persian(false), false);
        assert_eq!("Это персидский текст?".has_persian(false), false);
        assert_eq!("这是波斯文字吗?".has_persian(false), false);
        assert_eq!("".has_persian(false), false);
    }

    #[test]
    fn is_persian_test() {
        assert_eq!("این یک متن فارسی است؟".is_persian(false), true);
        assert_eq!(
            "آیا سیستم میتواند گزینه های دیگری را به اشتباه به عنوان متن فارسی تشخیص دهد؟"
                .is_persian(false),
            true
        );

        assert_eq!("Lorem Ipsum Test".is_persian(false), false);
        assert_eq!("これはペルシア語のテキストですか".is_persian(false), false);
        assert_eq!("Это персидский текст?".is_persian(false), false);
        assert_eq!("这是波斯文字吗?".is_persian(false), false);
        assert_eq!("هل هذا نص فارسي؟".is_persian(false), false);
        assert_eq!(
            "أكد رئيس اللجنة العسكرية الممثلة لحكومة الوفاق الوطني في ليبيا أحمد علي أبو شحمة، أن اللجنة لا تستطيع تنفيذ خطتها لإخراج العناصر الأجنبية من أراضي البلاد.".is_persian(false),
            false
        );
        assert_eq!("".is_persian(false), false);
        assert_eq!("مهدی".to_persian_chars().is_persian(false), true);
        assert_eq!("شاه".to_persian_chars().is_persian(false), true);
    }

    #[test]
    fn test_name() {
        assert_eq!("علي".to_persian_chars(), "علی");

        assert_eq!(String::from("علي").to_persian_chars(), "علی");

        let mut name = String::from("علي در اراك");
        name.to_persian_chars_mut();
        assert_eq!(name, "علی در اراک");

        let mut name = Cow::Borrowed("علي در اراك");
        assert_eq!(name.to_persian_chars(), "علی در اراک");
        name.to_persian_chars_mut();
        assert_eq!(name, Cow::<String>::Owned("علی در اراک".to_string()))
    }
}
