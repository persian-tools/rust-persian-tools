pub mod extracted_card_number;
use extracted_card_number::ExtractedCardNumber;

/// This function gets a string as input and extract iran standard bank card numbers.
/// persian chars for numbers are allowed "۱۲۳۴۵۶۷۸۹"
/// dash "-" and spaces " " in the middle of card numbers allowed
/// Example:
/// ```
/// use rust_persian_tools::extract_card_number::{
///     extract_card_number,
///     extracted_card_number::ExtractedCardNumber
/// };
/// assert_eq!(
///     extract_card_number("شماره کارتم رو برات نوشتم: ۵۰۲۲-2910-7۰۸۷-۳۴۶۶"),
///     vec![ExtractedCardNumber::new(
///         "5022291070873466",
///         "۵۰۲۲-2910-7۰۸۷-۳۴۶۶"
///     )],
/// )
/// ```
pub fn extract_card_number(inp: impl AsRef<str>) -> Vec<ExtractedCardNumber> {
    let inp = inp.as_ref();

    let mut result: Vec<ExtractedCardNumber> = Vec::new();

    let mut base = String::with_capacity(16);
    let mut pure = String::with_capacity(19); //with three "-" max chars inside

    for c in inp.chars() {
        let converted = convert(c);

        if converted.is_digit(10) {
            base.push(converted);
            pure.push(c);
        } else {
            if c != '-' && c != ' ' {
                base.clear();
                pure.clear();
            } else {
                if !pure.is_empty() {
                    pure.push(c);
                }
            }
        }

        if base.len() == 16 {
            result.push(ExtractedCardNumber::new(base.clone(), pure.clone()));
        }
    }

    result
}

fn convert(c: char) -> char {
    match c {
        '۰' => '0',
        '۱' => '1',
        '۲' => '2',
        '۳' => '3',
        '۴' => '4',
        '۵' => '5',
        '۶' => '6',
        '۷' => '7',
        '۸' => '8',
        '۹' => '9',
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(1, 1);
        let input = "شماره کارتم رو برات نوشتم:
6219-8610-3452-9007
اینم یه شماره کارت دیگه ای که دارم
50222910708734666
۵۰۲۲۲۹۱۰۸۱۸۷۳۴۶۶
۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶";
        assert_eq!(
            extract_card_number(input),
            vec![
                ExtractedCardNumber::new("6219861034529007", "6219-8610-3452-9007"),
                ExtractedCardNumber::new("5022291070873466", "5022291070873466"),
                ExtractedCardNumber::new("5022291081873466", "۵۰۲۲۲۹۱۰۸۱۸۷۳۴۶۶"),
                ExtractedCardNumber::new("5022291070873466", "۵۰۲۲-۲۹۱۰-۷۰۸۷-۳۴۶۶"),
            ]
        );

        // mixed
        assert_eq!(
            extract_card_number("شماره کارتم رو برات نوشتم: ۵۰۲۲-2910-7۰۸۷-۳۴۶۶"),
            vec![ExtractedCardNumber::new(
                "5022291070873466",
                "۵۰۲۲-2910-7۰۸۷-۳۴۶۶"
            )],
        )
    }
}
