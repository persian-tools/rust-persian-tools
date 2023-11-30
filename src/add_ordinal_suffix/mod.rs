use std::borrow::Cow;

///  Add Ordinal suffix to numbers <br>
///  Example: <br>
///  --- input: چهل و سه <br>
///  --- output: چهل و سوم
pub fn add_ordinal_suffix(number: impl AsRef<str>) -> String {
    let number = number.as_ref();
    if number.ends_with('ی') {
        number.to_string() + " اُم"
    } else if number.ends_with("سه") {
        number[0..number.len() - 2].to_string() + "وم"
    } else {
        number.to_string() + "م"
    }
}

pub fn add_ordinal_suffix_mut(number: &mut String) {
    if number.ends_with('ی') {
        number.push_str(" اُم")
    } else if number.ends_with("سه") {
        number.pop();
        number.push_str("وم")
    } else {
        number.push('م')
    }
}

pub trait AddOrdinalSuffix {
    fn add_persian_ordinal_suffix(&self) -> String;
}

pub trait AddOrdinalSuffixMut {
    fn add_persian_ordinal_suffix_mut(&mut self);
}

impl AddOrdinalSuffix for str {
    fn add_persian_ordinal_suffix(&self) -> String {
        add_ordinal_suffix(self)
    }
}

impl AddOrdinalSuffix for String {
    fn add_persian_ordinal_suffix(&self) -> String {
        add_ordinal_suffix(self)
    }
}

impl AddOrdinalSuffixMut for String {
    fn add_persian_ordinal_suffix_mut(&mut self) {
        add_ordinal_suffix_mut(self)
    }
}

impl AddOrdinalSuffix for Cow<'_, str> {
    fn add_persian_ordinal_suffix(&self) -> String {
        match self {
            Cow::Borrowed(borrowed) => add_ordinal_suffix(borrowed),
            Cow::Owned(owned) => add_ordinal_suffix(owned),
        }
    }
}

impl AddOrdinalSuffixMut for Cow<'_, str> {
    fn add_persian_ordinal_suffix_mut(&mut self) {
        add_ordinal_suffix_mut(self.to_mut());
    }
}

#[cfg(test)]
mod add_ordinal_suffix_tests {
    use super::*;

    #[test]
    fn convert() {
        assert_eq!("چهل و سه".add_persian_ordinal_suffix(), "چهل و سوم");
        assert_eq!(
            String::from("چهل و پنج").add_persian_ordinal_suffix(),
            "چهل و پنجم"
        );
        assert_eq!("سی".add_persian_ordinal_suffix(), "سی اُم");
        assert_eq!(
            Cow::from("بیست و یک").add_persian_ordinal_suffix(),
            "بیست و یکم"
        );
    }

    #[test]
    fn in_place() {
        let mut number = "چهل و سه".to_string();
        number.add_persian_ordinal_suffix_mut();
        assert_eq!(number, "چهل و سوم");

        let mut number = Cow::from("چهل و پنج");
        assert_eq!(number, Cow::Borrowed("چهل و پنج"));
        number.add_persian_ordinal_suffix_mut();
        assert_eq!(number, Cow::Owned("چهل و پنجم".to_string()) as Cow<'_, str>);

        let mut number = Cow::from("سی".to_string());
        assert_eq!(number, Cow::Owned("سی".to_string()) as Cow<'_, str>);
        number.add_persian_ordinal_suffix_mut();
        assert_eq!(number, Cow::Owned("سی اُم".to_string()) as Cow<'_, str>);
    }
}
