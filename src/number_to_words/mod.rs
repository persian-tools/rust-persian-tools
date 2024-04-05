pub mod error;
mod get_word;
mod three_digits;

use crate::number_to_words::get_word::WITH_3N_ZEROS;
use crate::number_to_words::three_digits::three_digit_number_to_words;
use std::cmp::Ordering;
use std::num::IntErrorKind;

use crate::commas::remove_commas::remove_commas;

pub use self::error::NumberToWordsError;

/// Convert integer number to persian words.\
/// If you want to pass string as input see: [number_to_words_str]
/// ```
/// # use rust_persian_tools::number_to_words::number_to_words;
/// assert_eq!(number_to_words(33), Ok("سی و سه".to_string()));
/// ```
pub fn number_to_words(input: i64) -> Result<String, NumberToWordsError> {
    let mut result = String::new();

    let mut num: u64 = match input.cmp(&0) {
        Ordering::Less => {
            result.push_str("منفی ");
            let num = input.abs().try_into();
            match num {
                Ok(v) => v,
                Err(_e) => return Err(NumberToWordsError::Unknown(input.to_string())),
            }
        }
        Ordering::Equal => return Ok("صفر".to_string()),
        Ordering::Greater => {
            let num = input.abs().try_into();
            match num {
                Ok(v) => v,
                Err(_e) => return Err(NumberToWordsError::Unknown(input.to_string())),
            }
        }
    };

    let mut chunks = Vec::<u64>::new();
    while num != 0 {
        let next = num % 1000;
        num /= 1000;
        chunks.push(next);
    }
    chunks.reverse();
    let mut l = chunks.len();
    for c in chunks {
        if c == 0 {
            continue;
        }
        l -= 1;
        result.push_str(three_digit_number_to_words(c)?.as_str());
        result.push(' ');
        result.push_str(WITH_3N_ZEROS[l]);
        result.push_str(" و ");
    }

    // remove " و " at the end
    result.remove(result.len() - 3);
    Ok(result.trim().to_owned())
}

/// Convert integer number to persian words.
/// ### Example:
/// ```
/// # use rust_persian_tools::number_to_words::number_to_words_str;
/// assert_eq!(
///     number_to_words_str("33"),
///     Ok("سی و سه".to_string())
/// );
/// ```
/// If you want to pass integer use: [number_to_words]
pub fn number_to_words_str(num: impl AsRef<str>) -> Result<String, NumberToWordsError> {
    let num = num.as_ref();

    match remove_commas(num).parse::<i64>() {
        Ok(value) => Ok(number_to_words(value)?),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => Err(NumberToWordsError::EmptyString),
            IntErrorKind::InvalidDigit => Err(NumberToWordsError::InvalidInteger(num.to_string())),
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                Err(NumberToWordsError::Overflow(num.to_string()))
            }
            _ => Err(NumberToWordsError::Unknown(num.to_string())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_to_word_test() {
        assert_eq!(number_to_words(0), Ok("صفر".to_string()));
        assert_eq!(number_to_words(4), Ok("چهار".to_string()));
        assert_eq!(number_to_words(33), Ok("سی و سه".to_string()));
        assert_eq!(number_to_words(123), Ok("صد و بیست و سه".to_string()));
        assert_eq!(number_to_words(103), Ok("صد و سه".to_string()));
        assert_eq!(
            number_to_words(3825),
            Ok("سه هزار و هشت صد و بیست و پنج".to_string())
        );
        assert_eq!(number_to_words(-4), Ok("منفی چهار".to_string()));
        assert_eq!(number_to_words(30000000000), Ok("سی میلیارد".to_string()));
        assert_eq!(
            number_to_words(987654321),
            Ok(
                "نه صد و هشتاد و هفت میلیون و شش صد و پنجاه و چهار هزار و سیصد و بیست و یک"
                    .to_string()
            )
        );

        assert_eq!(
            number_to_words(9006199254740992),
            Ok("نه کوآدریلیون و شش تریلیون و صد و نود و نه میلیارد و دویست و پنجاه و چهار میلیون و هفت صد و چهل هزار و نه صد و نود و دو".to_string()),
        );
    }

    #[test]
    fn number_to_word_str_test() {
        assert_eq!(number_to_words_str("33"), Ok("سی و سه".to_string()));
        assert_eq!(
            number_to_words_str("8,356"),
            Ok("هشت هزار و سیصد و پنجاه و شش".to_string())
        );
        assert_eq!(
            number_to_words_str("500,443"),
            Ok("پانصد هزار و چهار صد و چهل و سه".to_string())
        );
    }

    #[test]
    fn number_to_word_errors_test() {
        // errors
        assert_eq!(
            number_to_words_str(""),
            Err(NumberToWordsError::EmptyString)
        );
        assert_eq!(
            number_to_words_str("123abc"),
            Err(NumberToWordsError::InvalidInteger("123abc".to_string()))
        );
        assert_eq!(
            number_to_words_str("3.14"),
            Err(NumberToWordsError::InvalidInteger("3.14".to_string()))
        );
        let long = "9999999999999999999999999999999999999999999999999";
        assert_eq!(
            number_to_words_str(long),
            Err(NumberToWordsError::Overflow(long.to_string()))
        );
    }
}
