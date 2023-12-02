/// Add Commas to numbers
/// 3000000 -> 3,000,000
/// Example:
/// ```rust
/// use rust_persian_tools::commas::add_commas::add_commas;
/// assert_eq!(add_commas("30000000"), "30,000,000");
/// ```
pub fn add_commas(str: impl Into<String>) -> String {
    let mut str: String = str.into();

    str = str.replace(',', "");
    let mut result = String::new();

    let end = str
        .chars()
        .position(|c| c == '.')
        .unwrap_or_else(|| str.chars().count());

    for (i, ch) in str.chars().enumerate() {
        if end == i {
            result.push_str(&str[end..str.chars().count()]);
            break;
        }
        if (end - i) % 3 == 0 && i != 0 {
            result.push(',')
        }
        result.push(ch);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_commas_test() {
        assert_eq!(add_commas("30000000"), "30,000,000");
        assert_eq!(add_commas("30000000"), "30,000,000");
        assert_eq!(add_commas("30,000,000"), "30,000,000");
        assert_eq!(add_commas("۳۰۰۰۰۰۰۰"), "۳۰,۰۰۰,۰۰۰"); // this is different from TS implementation

        assert_eq!(add_commas("30,000,000.02"), "30,000,000.02");
        assert_eq!(add_commas("12500.9"), "12,500.9");
        assert_eq!(add_commas("12500.9"), "12,500.9");
        assert_eq!(add_commas("51000.123456789"), "51,000.123456789");

        assert_eq!(add_commas("300"), "300");
        assert_eq!(add_commas("0"), "0");
    }
}
