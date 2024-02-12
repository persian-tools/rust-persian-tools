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
pub fn add_commas(number_str: impl AsRef<str>) -> String {
    let number_str = number_str.as_ref();
    let comma_less = |c: &char| c != &',';

    let mut end = number_str
        .chars()
        .filter(comma_less)
        .position(|c| c == '.')
        .unwrap_or_else(|| number_str.chars().filter(comma_less).count());

    let mut result = String::new();
    for (i, ch) in number_str.chars().filter(comma_less).enumerate() {
        if end == i {
            end += number_str.chars().filter(|c| c == &',').count();
            result.push_str(&number_str[end..number_str.chars().count()]);
            break;
        }
        if (end - i) % 3 == 0 && i != 0 {
            result.push(',')
        }
        result.push(ch);
    }

    result
}

/// Add Commas to numbers in place\
/// 3000000 -> 3,000,000\
/// Example:
/// ```rust
/// // function:
/// use rust_persian_tools::commas::add_commas::add_commas_mut;
/// let mut input = "30000000".to_string();
/// add_commas_mut(&mut input);
/// assert_eq!(input, "30,000,000");
/// ```
pub fn add_commas_mut(number_str: &mut String) {
    let comma_less = |c: &char| c != &',';
    let mut end = number_str
        .chars()
        .filter(comma_less)
        .position(|c| c == '.')
        .unwrap_or_else(|| number_str.chars().filter(comma_less).count());

    let mut i = 0;
    while i < end {
        if (end - i) % 3 == 0 && i != 0 {
            let c = number_str.chars().nth(i).unwrap(); // this is safe because i can be at maximum of string len
            if c != ',' {
                number_str.insert(i, ',');
            }
            end += 1;
            i += 1;
        }
        i += 1;
    }
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

    #[test]
    fn add_commas_mut_test() {
        let mut input = "30000000".to_string();
        add_commas_mut(&mut input);
        assert_eq!(input, "30,000,000");

        let mut input = "30,000,000".to_string();
        add_commas_mut(&mut input);
        assert_eq!(input, "30,000,000");

        let mut input = "30000000.02".to_string();
        add_commas_mut(&mut input);
        assert_eq!(input, "30,000,000.02");
    }
}
