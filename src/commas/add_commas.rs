/// Add Commas to numbers\
/// 3000000 -> 3,000,000\
/// Example:
/// ```rust
/// // function:
/// use rust_persian_tools::commas::add_commas::add_commas;
/// assert_eq!(add_commas("30000000"), "30,000,000");
///
/// // method:
/// use crate::rust_persian_tools::commas::add_commas::AddCommas;
/// assert_eq!("30000000".add_commas(), "30,000,000");
///
/// // method on your custom type:
/// struct YourCustomType;
/// impl AddCommas for YourCustomType{
///     fn add_commas(&self) -> String {
///         add_commas("hello")
///     }
/// }
/// ```
pub fn add_commas(inp: &str) -> String {
    let comma_less = |c: &char| c != &',';

    let mut end = inp
        .chars()
        .filter(comma_less)
        .position(|c| c == '.')
        .unwrap_or_else(|| inp.chars().filter(comma_less).count());

    let mut result = String::new();
    for (i, ch) in inp.chars().filter(comma_less).enumerate() {
        if end == i {
            end += inp.chars().filter(|c| c == &',').count();
            result.push_str(&inp[end..inp.chars().count()]);
            break;
        }
        if (end - i) % 3 == 0 && i != 0 {
            result.push(',')
        }
        result.push(ch);
    }

    result
}

pub trait AddCommas {
    fn add_commas(&self) -> String;
}

impl AddCommas for str {
    fn add_commas(&self) -> String {
        add_commas(self)
    }
}

impl AddCommas for String {
    fn add_commas(&self) -> String {
        add_commas(self)
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
