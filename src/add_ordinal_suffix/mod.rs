//! Add persian ordinal suffix to numbers.
//!
//! ### Example
//! ```rust
//! // Use functions
//! use rust_persian_tools::add_ordinal_suffix::{add_ordinal_suffix, add_ordinal_suffix_mut};
//! // Or use methods (on `str`, `String`, and `Cow<'_, str>`):
//! use rust_persian_tools::add_ordinal_suffix::{AddOrdinalSuffix, AddOrdinalSuffixMut};
//!
//! let input = "بیست و یک";
//! assert_eq!(add_ordinal_suffix(input), "بیست و یکم");
//! let mut input = input.to_string();
//! add_ordinal_suffix_mut(&mut input);
//! assert_eq!(input, "بیست و یکم");
//!
//! let input = "چهل و سه";
//! assert_eq!(input.add_persian_ordinal_suffix(), "چهل و سوم");
//! let mut input = input.to_string();
//! input.add_persian_ordinal_suffix_mut();
//! assert_eq!(input, "چهل و سوم");
//! ```

use std::borrow::Cow;

/// Add persian ordinal suffix to numbers.
///
/// Enable via `add-ordinal-suffix` Cargo feature.
///
/// ### Example
/// ```rust
/// use rust_persian_tools::add_ordinal_suffix::add_ordinal_suffix;
///
/// let input = "بیست و یک".to_string();
/// assert_eq!(add_ordinal_suffix(input), "بیست و یکم");
/// ```
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

/// Add persian ordinal suffix to numbers in-place.
///
/// Enable via `add-ordinal-suffix` Cargo feature.
///
/// ### Example
/// ```rust
/// use rust_persian_tools::add_ordinal_suffix::add_ordinal_suffix_mut;
///
/// let mut input = "سی".to_string();
/// add_ordinal_suffix_mut(&mut input);
/// assert_eq!(input, "سی اُم");
/// ```
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
