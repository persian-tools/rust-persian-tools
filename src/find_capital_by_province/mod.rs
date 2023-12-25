use crate::persian_chars::to_persian_chars;

/// Returns the capital name of province you enter
pub fn find_capital_by_province<S>(inp: S) -> Option<String>
where
    S: Into<String>,
{
    let r = match to_persian_chars(inp).as_str() {
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
    };
    Some(r.to_string())
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
        assert_eq!(find_capital_by_province("تهران"), Some("تهران".to_string()));
        assert_eq!(find_capital_by_province("مرکزی"), Some("اراک".to_string()));
        assert_eq!(
            find_capital_by_province("خراسان رضوی"),
            Some("مشهد".to_string())
        );
    }

    #[test]
    fn find_capital_by_province_arabic_char() {
        assert_eq!(
            find_capital_by_province("خراسان رضوي"),
            Some("مشهد".to_string())
        );
    }
}
