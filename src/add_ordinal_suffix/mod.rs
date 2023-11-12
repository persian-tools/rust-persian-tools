
///  Add Ordinal suffix to numbers <br>
///  Example: <br>
///  --- input: چهل و سه <br>
///  --- output: چهل و سوم
pub fn add_ordinal_suffix<S>(number: S) -> String where S: Into<String> {
    let number: String = number.into();

	if number.ends_with('ی'){
		return number + " اُم";
	}
	if number.ends_with("سه"){
        return number[0..number.len()-2].to_string() + "وم";
	}
	number + "م"
}


#[cfg(test)]
mod add_ordinal_suffix_tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add_ordinal_suffix("چهل و سه"),"چهل و سوم");
        assert_eq!(add_ordinal_suffix("چهل و پنج"),"چهل و پنجم");
        assert_eq!(add_ordinal_suffix("سی"),"سی اُم");
    }
}