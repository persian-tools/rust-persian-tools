mod constants;
mod errors;

use crate::{
    commas::add_commas::add_commas_mut,
    digits::{DigitsAr2En, DigitsEn2ArMut, DigitsEn2FaMut, DigitsFa2En},
    remove_ordinal_suffix::remove_ordinal_suffix,
};

use self::{
    constants::{get_magnitute_number, get_unit_number, NEGATIVE_PREFIX},
    errors::WordsToNumberError,
};

#[derive(Debug, PartialEq)]
pub enum Language {
    Arabic,
    Persian,
    English,
}

pub struct Options {
    pub digits: Language,
    pub add_commas: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            digits: Language::English,
            add_commas: false,
        }
    }
}

fn calculate(tokens: Vec<String>) -> Result<i64, WordsToNumberError> {
    let mut sum = 0;
    let mut is_negetive = false;

    for token in tokens {
        if token == NEGATIVE_PREFIX {
            // check negetive token
            is_negetive = true;
        } else if let Some(value) = get_unit_number(&token) {
            // if token is a valid number
            sum += value;
        } else if let Some(value) = get_magnitute_number(&token) {
            // if token is a magnitute valid number
            if sum == 0 {
                sum = *value;
            } else {
                sum *= value;
            }
        } else if let Ok(value) = token.digits_fa_to_en().digits_ar_to_en().parse::<i64>() {
            sum += value;
        } else {
            return Err(WordsToNumberError::InvalidUnit);
        }
    }

    if is_negetive {
        Ok(-sum)
    } else {
        Ok(sum)
    }
}

/// returns a i64 number if the givin input is a standard persian number text otherwise it would return a error
///
/// ordinal suffix is supported for example "منفی سی اُم"
///
/// if you need to change numbers format for example add commas or change numbers to arabic or persian as result you may use [words_to_number_str]
///
/// # Examples
///
/// ```
/// use rust_persian_tools::words_to_number::words_to_number;
///
/// assert_eq!(words_to_number("منفی سه هزار").unwrap(), -3000);
/// assert!(words_to_number("سلام چطوری").is_err());
/// ```
pub fn words_to_number(words: impl AsRef<str>) -> Result<i64, WordsToNumberError> {
    let words = words.as_ref().trim();

    if words.is_empty() {
        return Err(WordsToNumberError::EmptyInput);
    }

    // remove ordinal suffix from each word
    let tokens = words
        .replace("شیش صد", "ششصد")
        .replace("شش صد", "ششصد")
        .replace("هفت صد", "هفتصد")
        .replace("هشت صد", "هشتصد")
        .replace("نه صد", "نهصد")
        .split_ascii_whitespace()
        .filter(|word| *word != "و" && *word != "ام" && *word != "اُم")
        .map(remove_ordinal_suffix)
        .collect::<Vec<String>>();

    calculate(tokens)
}

/// returns a number as [String] if the givin input is a standard persian number text otherwise it would return a error
///
/// first you need to create a [Options] struct , you may also use ```Options::defualt()```
///
/// ordinal suffix is supported for example "منفی سی اُم"
///
/// if you just need the number as i64 you may use [words_to_number]
///
/// # Examples
///
/// ```
/// use rust_persian_tools::words_to_number::{Options,Language,words_to_number_str};
///
/// let option = Options {
///     digits: Language::Arabic,
///     add_commas: true,
/// };
/// assert_eq!(
///     words_to_number_str("دوازده هزار بیست دو", &option).unwrap(),
///     "١٢,٠٢٢"
/// );
/// ```
pub fn words_to_number_str(
    words: impl AsRef<str>,
    option: &Options,
) -> Result<String, WordsToNumberError> {
    let number = words_to_number(words)?;
    let mut result = format!("{number}");

    if option.add_commas {
        add_commas_mut(&mut result)
    }

    if option.digits == Language::Arabic {
        result.digits_en_to_ar_mut();
    } else if option.digits == Language::Persian {
        result.digits_en_to_fa_mut();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_to_number_test() {
        assert_eq!(words_to_number("منفی سه هزار").unwrap(), -3000);
        assert_eq!(words_to_number("دوازده هزار بیست دو").unwrap(), 12022);
        assert_eq!(words_to_number("دوازده هزار و بیست و دو").unwrap(), 12022);
    }

    #[test]
    fn words_to_number_with_comma_test() {
        let option = Options {
            digits: Language::English,
            add_commas: true,
        };

        assert_eq!(
            words_to_number_str("دوازده هزار بیست دو", &option).unwrap(),
            "12,022"
        );
        assert_eq!(
            words_to_number_str("دوازده هزار و بیست و دو", &option).unwrap(),
            "12,022"
        );
    }

    #[test]
    fn words_to_number_arabic() {
        let option = Options {
            digits: Language::Arabic,
            add_commas: false,
        };

        assert_eq!(
            words_to_number_str("منفی سه هزار", &option).unwrap(),
            "-٣٠٠٠"
        );
        assert_eq!(
            words_to_number_str("سه هزار دویست و دوازده", &option).unwrap(),
            "٣٢١٢"
        );
        assert_eq!(
            words_to_number_str("دوازده هزار بیست دو", &option).unwrap(),
            "١٢٠٢٢"
        );
        assert_eq!(
            words_to_number_str("چهارصد پنجاه هزار", &option).unwrap(),
            "٤٥٠٠٠٠"
        );
    }

    #[test]
    fn words_to_number_with_comma_arabic() {
        let option = Options {
            digits: Language::Arabic,
            add_commas: true,
        };

        assert_eq!(
            words_to_number_str("دوازده هزار بیست دو", &option).unwrap(),
            "١٢,٠٢٢"
        );
        assert_eq!(
            words_to_number_str("دوازده هزار و بیست و دو", &option).unwrap(),
            "١٢,٠٢٢"
        );
        assert_eq!(
            words_to_number_str("چهارصد پنجاه هزار", &option).unwrap(),
            "٤٥٠,٠٠٠"
        );
    }

    #[test]
    fn words_to_number_with_ordinal_words() {
        let option_comma_fa = Options {
            digits: Language::Persian,
            add_commas: true,
        };

        assert_eq!(
            words_to_number_str("منفی ۳ هزار", &option_comma_fa).unwrap(),
            "-۳,۰۰۰"
        );
        assert_eq!(
            words_to_number_str("منفی 3 هزار و 200", &option_comma_fa).unwrap(),
            "-۳,۲۰۰"
        );
        assert_eq!(
            words_to_number_str("منفی سه هزارمین", &option_comma_fa).unwrap(),
            "-۳,۰۰۰"
        );

        assert_eq!(words_to_number("منفی سه هزارم").unwrap(), -3000);
        assert_eq!(words_to_number("سی و سوم").unwrap(), 33);
        assert_eq!(words_to_number("منفی سی اُم").unwrap(), -30);
    }

    #[test]
    fn words_to_number_fail_cases() {
        assert!(words_to_number("سلام چطوری").is_err());
        assert!(words_to_number("").is_err());
    }
}
