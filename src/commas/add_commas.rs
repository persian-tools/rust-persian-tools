/// Add Commas to numbers
/// 3000000 -> 3,000,000
pub fn add_commas<S>(str: S) -> String
where
    S: Into<String>,
{
    let mut str: String = str.into();
    str = str.replace(',', "");

    let mut end: Option<&str> = None;
    let s = str.clone();
    if str.contains('.') {
        let sp: Vec<&str> = s.split('.').collect();
        str = sp.get(0).unwrap().to_string();
        end = Some(sp.get(1).unwrap());
    }

    let mut result = String::new();
    for (i, ch) in str.chars().rev().enumerate() {
        if i % 3 == 0 {
            if i != 0 {
                result.push(',')
            }
        }
        result.push(ch);
    }
    let result = result.chars().rev().collect::<String>();
    match end {
        None => result,
        Some(x) => format!("{}.{}", result, x),
    }
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
