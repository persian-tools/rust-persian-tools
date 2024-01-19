use std::borrow::Cow;

pub mod chars {
    pub static AR_NUMBER: &str = "٠١٢٣٤٥٦٧٨٩"; // tick
    pub static AR_TEXT: &str =
        "ابتثجحخدذرزسشصضطظعغفقكلمنهويآةى؟ؠءأؤإ ؘ ؙ ؚ؛ ً ٌ ٍ َ ُ ِ ّ ْ ٓ ٔ ٕ ٖ ٗ ٘ ٙ ٚ ٛ ٝ ٞ ٟ٠١٢٣٤٥٦٧٨٩";
}

/// Return true if the entered string includes arabic characters
pub fn has_arabic(input: impl AsRef<str>) -> bool {
    input
        .as_ref()
        .chars()
        .any(|char| char != ' ' && chars::AR_TEXT.contains(char))
}

/// Return true if the entered string does not include other-language characters.
pub fn is_arabic(input: impl AsRef<str>) -> bool {
    !input.as_ref().is_empty()
        && input
            .as_ref()
            .chars()
            .all(|char| chars::AR_TEXT.contains(char))
}

/// Description: Replaces all instances of ی and ک with  ي and ك,
/// respectively. It should not make any changes to Persian text
/// surrounded by appropriate templates.
pub fn to_arabic_chars(input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .replace('ک', "ك")
        .replace('ی', "ي")
        .replace('ی', "ى")
}

pub fn to_arabic_chars_mut<I>(mut input: I)
where
    I: AsMut<str> + AsRef<str>,
{
    input
        .as_ref()
        .match_indices('ی')
        .chain(input.as_ref().match_indices('ک'))
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|index|
            // SAFETY:
            // We do not change the length of `&mut str`
            // We replace two bytes values only.
            unsafe {
            // Bytes for 'ی': [0xDB, 0x8C]
            // Bytes for 'ک': [0xDA, 0xA9]
            let next_byte = input.as_mut().as_bytes_mut().get_unchecked_mut(index + 1);

            match next_byte {
                // Characters 'ی' will be converted to 'ي'
                0x8C => {
                    *next_byte = 0x8A;
                    *input.as_mut().as_bytes_mut().get_unchecked_mut(index) = 0xD9;
                }
                // Character 'ک' will be converted to 'ك'
                0xA9 => {
                    *next_byte = 0x83;
                    *input.as_mut().as_bytes_mut().get_unchecked_mut(index) = 0xD9;
                }
                other => {
                    unreachable!("Second byte: {other}")
                }
            }
        });
}

pub trait ToArabicChars {
    fn to_arabic_chars(&self) -> String;
}

pub trait ToArabicCharsMut {
    fn to_arabic_chars_mut(&mut self);
}

pub trait HasArabic {
    fn has_arabic(&self) -> bool;
}

pub trait IsArabic {
    fn is_arabic(&self) -> bool;
}

impl ToArabicChars for str {
    fn to_arabic_chars(&self) -> String {
        to_arabic_chars(self)
    }
}

impl ToArabicChars for String {
    fn to_arabic_chars(&self) -> String {
        to_arabic_chars(self)
    }
}

impl ToArabicChars for Cow<'_, str> {
    fn to_arabic_chars(&self) -> String {
        to_arabic_chars(self)
    }
}

impl ToArabicCharsMut for String {
    fn to_arabic_chars_mut(&mut self) {
        to_arabic_chars_mut(self)
    }
}

impl ToArabicCharsMut for Cow<'_, str> {
    fn to_arabic_chars_mut(&mut self) {
        to_arabic_chars_mut(self.to_mut())
    }
}

impl HasArabic for str {
    fn has_arabic(&self) -> bool {
        has_arabic(self)
    }
}

impl HasArabic for String {
    fn has_arabic(&self) -> bool {
        has_arabic(self)
    }
}

impl HasArabic for Cow<'_, str> {
    fn has_arabic(&self) -> bool {
        has_arabic(self)
    }
}

impl IsArabic for str {
    fn is_arabic(&self) -> bool {
        is_arabic(self)
    }
}

impl IsArabic for String {
    fn is_arabic(&self) -> bool {
        is_arabic(self)
    }
}

impl IsArabic for Cow<'_, str> {
    fn is_arabic(&self) -> bool {
        is_arabic(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_arabic_test() {
        assert_eq!("هل هذا نص عربي؟".has_arabic(), true);
        assert_eq!("هل هذا نص عربي؟".has_arabic(), true);
        assert_eq!(
            "هل يمكن للنظام أن يتعرف عن طريق الخطأ على الخيارات الأخرى كنص عربي؟".has_arabic(),
            true
        );
        assert_eq!("This text includes عربي".has_arabic(), true);
        assert_eq!("Это персидский ص текст?".has_arabic(), true);
        assert_eq!(
            "أكد رئيس اللجنة العسكرية الممثلة لحكومة الوفاق أراضي البلاد.".has_arabic(),
            true
        );

        assert_eq!("Lorem Ipsum Test".has_arabic(), false);
        assert_eq!("これはペルシア語のテキストですか".has_arabic(), false);
        assert_eq!("Это персидский текст?".has_arabic(), false);
        assert_eq!("这是波斯文字吗?".has_arabic(), false);
        assert_eq!("".has_arabic(), false);
    }

    #[test]
    fn is_arabic_test() {
        assert_eq!("هل هذا نص عربي؟".is_arabic(), true);
        assert_eq!(
            "هل يمكن للنظام أن يتعرف عن طريق الخطأ على الخيارات الأخرى كنص عربي؟".is_arabic(),
            true
        );

        assert_eq!("Lorem Ipsum Test".is_arabic(), false);
        assert_eq!("これはペルシア語のテキストですか".is_arabic(), false);
        assert_eq!("Это персидский текст?".is_arabic(), false);
        assert_eq!("这是波斯文字吗?".is_arabic(), false);
        assert_eq!("این متن عربی است".is_arabic(), false);
        assert_eq!(
            "آیا سیستم میتواند گزینه های دیگری را به اشتباه به عنوان متن فارسی تشخیص دهد؟"
                .is_arabic(),
            false
        );
        assert_eq!("".is_arabic(), false);
        assert_eq!("مهدی".to_arabic_chars().is_arabic(), true);
        assert_eq!("شاه".to_arabic_chars().is_arabic(), true);
    }

    #[test]
    fn test_name() {
        assert_eq!("علی".to_arabic_chars(), "علي");

        assert_eq!(String::from("علی").to_arabic_chars(), "علي");

        let mut name = String::from("علی در اراک");
        name.to_arabic_chars_mut();
        assert_eq!(name, "علي در اراك");

        let mut name = Cow::Borrowed("علی در اراک");
        assert_eq!(name.to_arabic_chars(), "علي در اراك");
        name.to_arabic_chars_mut();
        assert_eq!(name, Cow::<String>::Owned("علي در اراك".to_string()))
    }
}
