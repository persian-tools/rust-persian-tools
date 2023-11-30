/// Remove all commas in string
/// 3,000,000 -> 3000000
pub fn remove_commas<S>(str: S) -> String
where
    S: Into<String>,
{
    let str: String = str.into();
    str.replace(",", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_commas_test() {
        assert_eq!(remove_commas("30,000,000"), "30000000".to_string());
    }
}
