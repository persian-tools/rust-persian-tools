mod error;
mod get_word;

use crate::number_to_words::get_word::*;
use std::num::IntErrorKind;

use crate::commas::remove_commas::remove_commas;

use self::error::NumberToWordsError;

/// Convert integer number to persian words.\
/// If you want to pass string as input see: [number_to_words_str]
/// ```
/// # use rust_persian_tools::number_to_words::number_to_words;
/// assert_eq!(number_to_words(33), "سی و سه");
/// ```
pub fn number_to_words(num: i64) -> String {
    let mut result = String::new();

    // handle negative
    if num == 0 {
        return "صفر".to_string();
    } else if num < 0 {
        result.push_str("منفی ");
    }
    let mut num: u64 = num.abs().try_into().unwrap(); // abs number is always positive

    let mut chunks = Vec::<u64>::new();
    while num != 0 {
        let next = num % 1000;
        num = num / 1000;
        chunks.push(next);
    }
    chunks.reverse();
    let mut l = chunks.len();
    for c in chunks {
        if c == 0 {
            continue;
        }
        l -= 1;
        result.push_str(three_digit_number_to_words(c).as_str());
        result.push_str(" ");
        result.push_str(big_numbers[l]);
        result.push_str(" و ");
    }

    result.pop();
    result.pop();
    result.pop();
    result = result.trim().to_owned();
    result
}

fn three_digit_number_to_words(num: u64) -> String {
    if num == 0 {
        return "".to_string();
    }

    let binding = digits_with_zeros(num);
    let mut digits = binding.iter();

    // on list
    if let Some(c) = get_word(num) {
        return c.to_string();
    }

    //          1              10           100
    match (digits.next(), digits.next(), digits.next()) {
        // some none none already handled before match statement
        (Some(d1), Some(d2), None) => {
            // example: 34
            // 10 -> 20 already handled
            return format!("{} و {}", get_word(*d2).unwrap(), get_word(*d1).unwrap());
        }
        (Some(d1), Some(d2), Some(d3)) => {
            match (d1, d2, d3) {
                (d1, 0, d3) => {
                    // example: 304
                    return format!("{} و {}", get_word(*d3).unwrap(), get_word(*d1).unwrap());
                }
                (0, d2, d3) => {
                    // example: 340
                    return format!("{} و {}", get_word(*d3).unwrap(), get_word(*d2).unwrap(),);
                }
                (d1, d2, d3) => {
                    // example: 345
                    return format!(
                        "{} و {} و {}",
                        get_word(*d3).unwrap(),
                        get_word(*d2).unwrap(),
                        get_word(*d1).unwrap()
                    );
                }
            }
        }
        _ => return "".to_string(),
    }
}

/// 123 -> [100, 20, 3]
fn digits_with_zeros(num: u64) -> Vec<u64> {
    let mut num = num;
    let mut digits = Vec::<u64>::new();
    let mut zeros: u32 = 0;
    while num != 0 {
        let next = num % 10;
        num = num / 10;
        digits.push(number_with_zeros(next, zeros));
        zeros += 1;
    }
    digits
}

/// 4,5 -> 400000
fn number_with_zeros(number: u64, zeros: u32) -> u64 {
    let mut number = number;
    for _ in 0..zeros {
        number *= 10;
    }
    return number;
}

// fn transform_word(num: i64) -> String{
//     if num == 0 { return "".to_string(); }
//     if (num <= 9) return getWord(num);
//     else if (num >= 11 && num <= 19) return getWord(num);

//     const residual = num <= 99 ? num % 10 : num % 100;
//     return residual === 0 ? getWord(num) : `${getWord(num - residual)} و ${transformeToWord(residual)}`;
// }

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
        Ok(value) => Ok(number_to_words(value)),
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
    fn number_to_words_three_digits() {
        assert_eq!(three_digit_number_to_words(0), "");
        assert_eq!(three_digit_number_to_words(4), "چهار");
        assert_eq!(three_digit_number_to_words(33), "سی و سه");
        assert_eq!(three_digit_number_to_words(30), "سی");
        assert_eq!(three_digit_number_to_words(303), "سیصد و سه");
        assert_eq!(three_digit_number_to_words(123), "صد و بیست و سه");
        assert_eq!(three_digit_number_to_words(103), "صد و سه");

        // error check
        for i in 0..999 {
            three_digit_number_to_words(i);
        }
    }

    #[test]
    fn number_to_word_test() {
        assert_eq!(number_to_words(0), "صفر");
        assert_eq!(number_to_words(4), "چهار");
        assert_eq!(number_to_words(33), "سی و سه");
        assert_eq!(number_to_words(123), "صد و بیست و سه");
        assert_eq!(number_to_words(103), "صد و سه");
        assert_eq!(number_to_words(3825), "سه هزار و هشت صد و بیست و پنج");
        assert_eq!(number_to_words(-4), "منفی چهار");
        assert_eq!(number_to_words(30000000000), "سی میلیارد");
        assert_eq!(
            number_to_words(987654321),
            "نه صد و هشتاد و هفت میلیون و شش صد و پنجاه و چهار هزار و سیصد و بیست و یک"
        );

        assert_eq!(
            number_to_words(9006199254740992),
            "نه کوآدریلیون و شش تریلیون و صد و نود و نه میلیارد و دویست و پنجاه و چهار میلیون و هفت صد و چهل هزار و نه صد و نود و دو",
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
