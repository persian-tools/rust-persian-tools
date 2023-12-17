///  bank card number validation
pub fn verify_card_number<S>(digits: S) -> bool
where
    S: Into<String>,
{
    let digits: String = digits.into();

    if digits.parse::<u64>().is_err() {
        return false;
    }
    if digits.len() < 16 {
        return false;
    } // TODO why not digits.len() != 16
    if digits[1..10].parse::<i32>().unwrap() == 0 {
        return false;
    }
    if digits[10..16].parse::<i32>().unwrap() == 0 {
        return false;
    }

    let mut sum = 0;
    for (i, digit) in digits.chars().enumerate() {
        let radix = if i % 2 == 0 { 2 } else { 1 };

        let sub_digit = (digit).to_string().parse::<i32>().unwrap() * radix;
        sum += if sub_digit > 9 {
            sub_digit - 9
        } else {
            sub_digit
        };
    }

    sum % 10 == 0
}

#[cfg(test)]
mod add_ordinal_suffix_tests {
    use super::*;

    #[test]
    fn bank_number_validation() {
        assert_eq!(verify_card_number("6037701689095443"), true);
        assert_eq!(verify_card_number("6219861034529007"), true);
        assert_eq!(verify_card_number("6219861034529008"), false);
        assert_eq!(verify_card_number("621986103452900"), false);
        assert_eq!(verify_card_number("0"), false);
    }
}
