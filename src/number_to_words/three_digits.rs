use crate::number_to_words::error::NumberToWordsError;

use super::get_word::get_defined_word;

/// Converts number to words for (0..1000)
pub(super) fn three_digit_number_to_words(num: u64) -> Result<String, NumberToWordsError> {
    if num > 999 {
        return Err(NumberToWordsError::Internal);
    }

    if num == 0 {
        return Ok("".to_string());
    }

    // already on list
    if let Ok(c) = get_defined_word(num) {
        return Ok(c.to_string());
    }

    let binding = digits_with_zeros(num);
    let mut digits = binding.iter();

    //          1              10           100
    Ok(match (digits.next(), digits.next(), digits.next()) {
        // (Some() None None) already handled above
        (Some(d1), Some(d2), None) => {
            // example: 34
            // 10 -> 20 already handled
            format!("{} و {}", get_defined_word(*d2)?, get_defined_word(*d1)?)
        }
        (Some(d1), Some(d2), Some(d3)) => {
            match (d1, d2, d3) {
                (d1, 0, d3) => {
                    // example: 304
                    format!("{} و {}", get_defined_word(*d3)?, get_defined_word(*d1)?)
                }
                (0, d2, d3) => {
                    // example: 340
                    format!("{} و {}", get_defined_word(*d3)?, get_defined_word(*d2)?)
                }
                (d1, d2, d3) => {
                    // example: 345
                    format!(
                        "{} و {} و {}",
                        get_defined_word(*d3)?,
                        get_defined_word(*d2)?,
                        get_defined_word(*d1)?
                    )
                }
            }
        }
        _ => "".to_string(),
    })
}

/// digits_with_zeros(123) -> [3, 20, 100]
fn digits_with_zeros(num: u64) -> Vec<u64> {
    let mut num = num;
    let mut digits = Vec::<u64>::new();
    let mut zeros: u32 = 0;
    while num != 0 {
        let next = num % 10;
        num /= 10;
        digits.push(number_with_zeros(next, zeros));
        zeros += 1;
    }
    digits
}

/// number_with_zeros(4, 5) -> 400000
fn number_with_zeros(number: u64, zeros: u32) -> u64 {
    let mut number = number;
    for _ in 0..zeros {
        number *= 10;
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_to_words_three_digits() {
        assert_eq!(three_digit_number_to_words(0), Ok("".to_string()));
        assert_eq!(three_digit_number_to_words(4), Ok("چهار".to_string()));
        assert_eq!(three_digit_number_to_words(33), Ok("سی و سه".to_string()));
        assert_eq!(three_digit_number_to_words(30), Ok("سی".to_string()));
        assert_eq!(
            three_digit_number_to_words(303),
            Ok("سیصد و سه".to_string())
        );
        assert_eq!(
            three_digit_number_to_words(123),
            Ok("صد و بیست و سه".to_string())
        );
        assert_eq!(three_digit_number_to_words(103), Ok("صد و سه".to_string()));

        // error check
        for i in 0..1000 {
            three_digit_number_to_words(i).unwrap();
        }
    }

    #[test]
    fn number_with_zeros_test() {
        assert_eq!(number_with_zeros(4, 5), 400000);
    }

    #[test]
    fn digits_with_zeros_test() {
        assert_eq!(digits_with_zeros(123), vec![3, 20, 100]);
    }
}
