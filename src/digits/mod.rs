//! Conversion between Persian, Arabic, and English numbers. (`digits` feature)
//!
//! ### Example
//! ```rust
//! use rust_persian_tools::digits::{DigitsFa2Ar, DigitsAr2En, DigitsEn2Fa, DigitsEn2ArMut};
//!
//! assert_eq!("۴۵".digits_fa_to_ar(), "٤٥");
//! assert_eq!("۴۵".digits_fa_to_ar().digits_ar_to_en(), "45");
//!
//! assert_eq!(3.14.digits_en_to_fa(), "۳.۱۴");
//! let mut n = (-6).to_string();
//! n.digits_en_to_ar_mut();
//! assert_eq!(n, "-٦");
//! ```

use std::borrow::Cow;
use std::ops::Range;

/// Convert Persian digits to English digits.
pub fn fa_to_en(input: impl AsRef<str>) -> String {
    // UTF-8 code points for Persian numbers: 1776..1785
    // UTF-8 code points for English numbers: 48..57
    // 48 - 1776 = -1728
    convert(input, 1776..1786, -1728)
}

/// Convert Persian digits to English  digits in-place.
pub fn fa_to_en_mut(input: &mut String) {
    convert_mut(input, 1776..1786, -1728)
}

/// convert English digits to Persian digits.
pub fn en_to_fa(input: impl AsRef<str>) -> String {
    // UTF-8 code points for English numbers: 48..57
    // UTF-8 code points for Persian numbers: 1776..1785
    // 1776 - 48 = 1728
    convert(input, 48..58, 1728)
}

/// Convert English digits to Persian digits in-place.
pub fn en_to_fa_mut(input: &mut String) {
    convert_mut(input, 48..58, 1728)
}

/// convert English digits to Arabic digits.
pub fn en_to_ar(input: impl AsRef<str>) -> String {
    // UTF-8 code points for English numbers: 48..57
    // UTF-8 code points for Arabic numbers: 1632..1641
    // 1632 - 48 = 1584
    convert(input, 48..58, 1584)
}

/// convert English digits to Arabic digits in-place.
pub fn en_to_ar_mut(input: &mut String) {
    convert_mut(input, 48..58, 1584)
}

/// convert Arabic digits to English digits.
pub fn ar_to_en(input: impl AsRef<str>) -> String {
    // UTF-8 code points for Arabic numbers: 1632..1641
    // UTF-8 code points for English numbers: 48..57
    // 48 - 1632 = -1584
    convert(input, 1632..1642, -1584)
}

/// convert Arabic digits to English digits in-place.
pub fn ar_to_en_mut(input: &mut String) {
    convert_mut(input, 1632..1642, -1584)
}

/// convert Persian digits to Arabic digits.
pub fn fa_to_ar(input: impl AsRef<str>) -> String {
    // UTF-8 code points for Persian numbers: 1776..1785
    // UTF-8 code points for Arabic numbers: 1632..1641
    // 1632 - 1776 = -144
    convert(input, 1776..1786, -144)
}

/// convert Persian digits to Arabic digits in-place.
pub fn fa_to_ar_mut(input: &mut String) {
    convert_mut(input, 1776..1786, -144)
}

/// convert Arabic digits to Persian digits.
pub fn ar_to_fa(input: impl AsRef<str>) -> String {
    // UTF-8 code points for Arabic numbers: 1632..1641
    // UTF-8 code points for Persian numbers: 1776..1785
    // 1776 - 1632 = 144
    convert(input, 1632..1642, 144)
}

/// convert Arabic digits to Persian digits in-place.
pub fn ar_to_fa_mut(input: &mut String) {
    convert_mut(input, 1632..1642, 144)
}

/// Conversion for Arabic to Persian digits.
pub trait DigitsAr2Fa {
    fn digits_ar_to_fa(&self) -> String;
}

/// Mutable conversion for Arabic to Persian digits.
pub trait DigitsAr2FaMut {
    fn digits_ar_to_fa_mut(&mut self);
}

/// Conversion for Arabic to English digits.
pub trait DigitsAr2En {
    fn digits_ar_to_en(&self) -> String;
}

/// Mutable conversion for Arabic to English digits.
pub trait DigitsAr2EnMut {
    fn digits_ar_to_en_mut(&mut self);
}

/// Conversion for Persian to Arabic digits.
pub trait DigitsFa2Ar {
    fn digits_fa_to_ar(&self) -> String;
}

/// Mutable conversion for Persian to Arabic digits.
pub trait DigitsFa2ArMut {
    fn digits_fa_to_ar_mut(&mut self);
}

/// Conversion for Persian to English digits.
pub trait DigitsFa2En {
    fn digits_fa_to_en(&self) -> String;
}

/// Mutable conversion for Persian to English digits.
pub trait DigitsFa2EnMut {
    fn digits_fa_to_en_mut(&mut self);
}

/// Conversion for English to Arabic digits.
pub trait DigitsEn2Ar {
    fn digits_en_to_ar(&self) -> String;
}

/// Mutable conversion for English to Arabic digits.
pub trait DigitsEn2ArMut {
    fn digits_en_to_ar_mut(&mut self);
}

/// Conversion for English to Persian digits.
pub trait DigitsEn2Fa {
    fn digits_en_to_fa(&self) -> String;
}

/// Mutable conversion for English to Persian digits.
pub trait DigitsEn2FaMut {
    fn digits_en_to_fa_mut(&mut self);
}

impl DigitsAr2Fa for str {
    fn digits_ar_to_fa(&self) -> String {
        ar_to_fa(self)
    }
}

impl DigitsAr2Fa for String {
    fn digits_ar_to_fa(&self) -> String {
        ar_to_fa(self)
    }
}

impl DigitsAr2Fa for Cow<'_, str> {
    fn digits_ar_to_fa(&self) -> String {
        ar_to_fa(self)
    }
}

impl DigitsAr2FaMut for String {
    fn digits_ar_to_fa_mut(&mut self) {
        ar_to_fa_mut(self)
    }
}

impl DigitsAr2FaMut for Cow<'_, str> {
    fn digits_ar_to_fa_mut(&mut self) {
        ar_to_fa_mut(self.to_mut())
    }
}

impl DigitsAr2En for str {
    fn digits_ar_to_en(&self) -> String {
        ar_to_en(self)
    }
}

impl DigitsAr2En for String {
    fn digits_ar_to_en(&self) -> String {
        ar_to_en(self)
    }
}

impl DigitsAr2En for Cow<'_, str> {
    fn digits_ar_to_en(&self) -> String {
        ar_to_en(self)
    }
}

impl DigitsAr2EnMut for String {
    fn digits_ar_to_en_mut(&mut self) {
        ar_to_en_mut(self)
    }
}

impl DigitsAr2EnMut for Cow<'_, str> {
    fn digits_ar_to_en_mut(&mut self) {
        ar_to_en_mut(self.to_mut())
    }
}

impl DigitsFa2Ar for str {
    fn digits_fa_to_ar(&self) -> String {
        fa_to_ar(self)
    }
}

impl DigitsFa2Ar for String {
    fn digits_fa_to_ar(&self) -> String {
        fa_to_ar(self)
    }
}

impl DigitsFa2Ar for Cow<'_, str> {
    fn digits_fa_to_ar(&self) -> String {
        fa_to_ar(self)
    }
}

impl DigitsFa2ArMut for String {
    fn digits_fa_to_ar_mut(&mut self) {
        fa_to_ar_mut(self)
    }
}

impl DigitsFa2ArMut for Cow<'_, str> {
    fn digits_fa_to_ar_mut(&mut self) {
        fa_to_ar_mut(self.to_mut())
    }
}

impl DigitsFa2En for str {
    fn digits_fa_to_en(&self) -> String {
        fa_to_en(self)
    }
}

impl DigitsFa2En for String {
    fn digits_fa_to_en(&self) -> String {
        fa_to_en(self)
    }
}

impl DigitsFa2En for Cow<'_, str> {
    fn digits_fa_to_en(&self) -> String {
        fa_to_en(self)
    }
}

impl DigitsFa2EnMut for String {
    fn digits_fa_to_en_mut(&mut self) {
        fa_to_en_mut(self)
    }
}

impl DigitsFa2EnMut for Cow<'_, str> {
    fn digits_fa_to_en_mut(&mut self) {
        fa_to_en_mut(self.to_mut())
    }
}

impl DigitsEn2Ar for str {
    fn digits_en_to_ar(&self) -> String {
        en_to_ar(self)
    }
}

impl DigitsEn2Ar for String {
    fn digits_en_to_ar(&self) -> String {
        en_to_ar(self)
    }
}

impl DigitsEn2Ar for Cow<'_, str> {
    fn digits_en_to_ar(&self) -> String {
        en_to_ar(self)
    }
}

impl DigitsEn2ArMut for String {
    fn digits_en_to_ar_mut(&mut self) {
        en_to_ar_mut(self)
    }
}

impl DigitsEn2ArMut for Cow<'_, str> {
    fn digits_en_to_ar_mut(&mut self) {
        en_to_ar_mut(self.to_mut())
    }
}

impl DigitsEn2Ar for u8 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for i8 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for u16 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for i16 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for u32 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for i32 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for u64 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for i64 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for u128 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for i128 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for usize {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for isize {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for f32 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Ar for f64 {
    fn digits_en_to_ar(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_ar_mut();
        string
    }
}

impl DigitsEn2Fa for str {
    fn digits_en_to_fa(&self) -> String {
        en_to_fa(self)
    }
}

impl DigitsEn2Fa for String {
    fn digits_en_to_fa(&self) -> String {
        en_to_fa(self)
    }
}

impl DigitsEn2Fa for Cow<'_, str> {
    fn digits_en_to_fa(&self) -> String {
        en_to_fa(self)
    }
}

impl DigitsEn2FaMut for String {
    fn digits_en_to_fa_mut(&mut self) {
        en_to_fa_mut(self)
    }
}

impl DigitsEn2FaMut for Cow<'_, str> {
    fn digits_en_to_fa_mut(&mut self) {
        en_to_fa_mut(self.to_mut())
    }
}

impl DigitsEn2Fa for u8 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for i8 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for u16 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for i16 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for u32 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for i32 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for u64 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for i64 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for u128 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for i128 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for usize {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for isize {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for f32 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

impl DigitsEn2Fa for f64 {
    fn digits_en_to_fa(&self) -> String {
        let mut string = self.to_string();
        string.digits_en_to_fa_mut();
        string
    }
}

#[inline]
fn convert(input: impl AsRef<str>, range: Range<u32>, diff: i64) -> String {
    let mut index = 0;
    let mut result = String::new();
    input
        .as_ref()
        .match_indices(|code| range.contains(&(code as u32)))
        .map(|(index, char_str)| unsafe { (index, char_str.chars().next().unwrap_unchecked()) })
        .for_each(|(char_index, old_char)| unsafe {
            result += input.as_ref().get_unchecked(index..char_index);
            let new_char = char::from_u32_unchecked((old_char as i64 + diff) as u32);
            result.push(new_char);
            index = char_index + old_char.len_utf8();
        });
    result += unsafe { input.as_ref().get_unchecked(index..) };
    result
}

#[inline]
fn convert_mut(input: &mut String, range: Range<u32>, diff: i64) {
    input
        .rmatch_indices(|code| range.contains(&(code as u32)))
        .map(|(index, char_str)| unsafe { (index, char_str.chars().next().unwrap_unchecked()) })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(char_index, old_char)| unsafe {
            let char = char::from_u32_unchecked((old_char as i64 + diff) as u32);
            input.remove(char_index);
            input.insert(char_index, char);
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_fa_to_en_test() {
        assert_eq!("123۴۵۶".digits_fa_to_en(), "123456");
        let mut n = "۸۹123۴۵".to_string();
        n.digits_fa_to_en_mut();
        assert_eq!(n, "8912345");
        let mut n = Cow::Borrowed("۰۱۲۳۴۵۶۷۸۹");
        n.digits_fa_to_en_mut();
        assert_eq!(n, Cow::<str>::Owned("0123456789".to_string()));
    }

    #[test]
    fn digits_en_to_fa_test() {
        let mut n = "123۴۵۶".to_string();
        n.digits_en_to_fa_mut();
        assert_eq!(n, "۱۲۳۴۵۶");
        assert_eq!(123i8.digits_en_to_fa(), "۱۲۳");
        assert_eq!(1234567891usize.digits_en_to_fa(), "۱۲۳۴۵۶۷۸۹۱");
        assert_eq!(3.14f64.digits_en_to_fa(), "۳.۱۴");
        assert_eq!("٤٥٦".to_string().digits_en_to_fa(), "٤٥٦");
        let mut n = Cow::Borrowed("123۴۵۶");
        n.digits_en_to_fa_mut();
        assert_eq!(n, Cow::<str>::Owned("۱۲۳۴۵۶".to_string()));
    }

    #[test]
    fn digits_en_to_ar_test() {
        let mut n = "123٤٥٦".to_string();
        n.digits_en_to_ar_mut();
        assert_eq!(n, "١٢٣٤٥٦");
        assert_eq!(123i8.digits_en_to_ar(), "١٢٣");
        assert_eq!(1234567891usize.digits_en_to_ar(), "١٢٣٤٥٦٧٨٩١");
        assert_eq!(3.14f64.digits_en_to_ar(), "٣.١٤");
        assert_eq!("۴۵۶".to_string().digits_en_to_ar(), "۴۵۶");
        let mut n = Cow::Borrowed("123٤٥٦");
        n.digits_en_to_ar_mut();
        assert_eq!(n, Cow::<str>::Owned("١٢٣٤٥٦".to_string()));
    }

    #[test]
    fn digits_ar_to_en_test() {
        assert_eq!("٠١٢٣٤٥٦٧٨٩".digits_ar_to_en(), "0123456789");
        let mut n = "89١٢٣4٥".to_string();
        n.digits_ar_to_en_mut();
        assert_eq!(n, "8912345");
        let mut n = Cow::Borrowed("0123۴۵۶789");
        n.digits_ar_to_en_mut();
        assert_eq!(n, Cow::<str>::Owned("0123۴۵۶789".to_string()));
    }

    #[test]
    fn digits_fa_to_ar_test() {
        assert_eq!("۰۱۲۳۴۵۶۷۸۹".digits_fa_to_ar(), "٠١٢٣٤٥٦٧٨٩");
        let mut n = "۱۷۸۲۳۴۰۵۶۹".to_string();
        n.digits_fa_to_ar_mut();
        assert_eq!(n, "١٧٨٢٣٤٠٥٦٩");
        assert_eq!("۷۸٤۲۳٤۴".to_string().digits_fa_to_ar(), "٧٨٤٢٣٤٤");
        let mut n = Cow::Borrowed("٤٤٤444۴۴۴");
        n.digits_fa_to_ar_mut();
        assert_eq!(n, Cow::<str>::Owned("٤٤٤444٤٤٤".to_string()));
    }

    #[test]
    fn digits_ar_to_fa_test() {
        assert_eq!("٠١٢٣٤٥٦٧٨٩".digits_ar_to_fa(), "۰۱۲۳۴۵۶۷۸۹");
        let mut n = "١٧٨٢٣٤٠٥٦٩".to_string();
        n.digits_ar_to_fa_mut();
        assert_eq!(n, "۱۷۸۲۳۴۰۵۶۹");
        assert_eq!("٧٨٤٢٣٤٤".to_string().digits_ar_to_fa(), "۷۸۴۲۳۴۴");
        let mut n = Cow::Borrowed("٤٤٤444٤٤٤");
        n.digits_ar_to_fa_mut();
        assert_eq!(n, Cow::<str>::Owned("۴۴۴444۴۴۴".to_string()));
    }
}
