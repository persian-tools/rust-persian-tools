use crate::persian_chars::to_persian_chars;

/// Returns the capital name of province you enter
/// # Example:
/// ```
/// use rust_persian_tools::find_capital_by_province::find_capital_by_province;
/// assert_eq!(find_capital_by_province("خراسان رضوي"), Some("مشهد"));
/// ```
pub fn find_capital_by_province(input: impl AsRef<str>) -> Option<&'static str> {
    let input = input.as_ref();

    Some(match to_persian_chars(input).as_str() {
        "آذربایجان شرقی" => "تبریز",
        "آذربایجان غربی" => "ارومیه",
        "اردبیل" => "اردبیل",
        "اصفهان" => "اصفهان",
        "البرز" => "کرج",
        "ایلام" => "ایلام",
        "بوشهر" => "بوشهر",
        "تهران" => "تهران",
        "چهارمحال و بختیاری" => "شهرکرد",
        "خراسان جنوبی" => "بیرجند",
        "خراسان رضوی" => "مشهد",
        "خراسان شمالی" => "بجنورد",
        "خوزستان" => "اهواز",
        "زنجان" => "زنجان",
        "سمنان" => "سمنان",
        "سیستان و بلوچستان" => "زاهدان",
        "فارس" => "شیراز",
        "قزوین" => "قزوین",
        "قم" => "قم",
        "کردستان" => "سنندج",
        "کرمان" => "کرمان",
        "کرمانشاه" => "کرمانشاه",
        "کهگیلویه و بویراحمد" => "یاسوج",
        "گلستان" => "گرگان",
        "گیلان" => "رشت",
        "لرستان" => "خرم آباد",
        "مازندران" => "ساری",
        "مرکزی" => "اراک",
        "هرمزگان" => "بندرعباس",
        "همدان" => "همدان",
        "یزد" => "یزد",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_capital_by_province_none() {
        assert_eq!(find_capital_by_province("random"), None);
    }

    #[test]
    fn find_capital_by_province_normal() {
        assert_eq!(find_capital_by_province("تهران"), Some("تهران"));
        assert_eq!(find_capital_by_province("مرکزی"), Some("اراک"));
        assert_eq!(find_capital_by_province("خراسان رضوی"), Some("مشهد"));
    }

    #[test]
    fn find_capital_by_province_arabic_char() {
        assert_eq!(find_capital_by_province("خراسان رضوي"), Some("مشهد"));
    }
}
