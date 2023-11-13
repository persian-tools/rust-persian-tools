

///  Validation of Iranian National Number(code-e Melli)
pub fn verify_iranian_national_id<S>(code: S) -> bool where S: Into<String> {
    let code: String = code.into();

    let code_length = code.len();
	if !(8..=10).contains(&code_length){ return false; }

    if code.parse::<u64>().is_err(){
        return false;
    }

    if code == "0000000000" { return false; }
    if code == "1111111111" { return false; }
    if code == "2222222222" { return false; }
    if code == "3333333333" { return false; }
    if code == "4444444444" { return false; }
    if code == "5555555555" { return false; }
    if code == "6666666666" { return false; }
    if code == "7777777777" { return false; }
    if code == "8888888888" { return false; }
    if code == "9999999999" { return false; }

	let code = ("00".to_owned() + &code)[code_length + 2 - 10..].to_string();
	if str::parse::<usize>(&code[3..9]).unwrap() == 0 { return false; }

	let last_number = str::parse::<usize>(code.chars().last().unwrap().to_string().as_str()).unwrap();

	let mut sum = 0;
    for i in 0..9{
		sum += str::parse::<usize>(&code[i..i+1]).unwrap() * (10 - i);
    }

	sum %= 11;

	(sum < 2 && last_number == sum) || (sum >= 2 && last_number == 11 - sum)

}


#[cfg(test)]
mod verify_iranian_national_id_tests {
    use super::*;


    #[test]
    fn check_falsy(){
        assert_eq!(verify_iranian_national_id(""), false);
        assert_eq!(verify_iranian_national_id("12345"), false);
        assert_eq!(verify_iranian_national_id("0"), false);
        assert_eq!(verify_iranian_national_id("000000"), false);
        assert_eq!(verify_iranian_national_id("12300000"), false);
        assert_eq!(verify_iranian_national_id("123000000"), false);
        assert_eq!(verify_iranian_national_id("1230000000"), false);
        assert_eq!(verify_iranian_national_id("0000000000"), false);
        assert_eq!(verify_iranian_national_id("4444444444"), false);
        assert_eq!(verify_iranian_national_id("9999999999"), false);
        assert_eq!(verify_iranian_national_id("0684159415"), false);
        assert_eq!(verify_iranian_national_id("1111111111"), false);
        assert_eq!(verify_iranian_national_id("079041a904"), false); // this is not in typescript version
    }

    #[test]
    fn check_truly(){
        assert_eq!(verify_iranian_national_id("11537027"), true);
        assert_eq!(verify_iranian_national_id("787833770"), true);
        assert_eq!(verify_iranian_national_id("1583250689"), true);
        assert_eq!(verify_iranian_national_id("0499370899"), true);
        assert_eq!(verify_iranian_national_id("0790419904"), true);
        assert_eq!(verify_iranian_national_id("0084575948"), true);
        assert_eq!(verify_iranian_national_id("0963695398"), true);
        assert_eq!(verify_iranian_national_id("0684159414"), true);
        assert_eq!(verify_iranian_national_id("0067749828"), true);
        assert_eq!(verify_iranian_national_id("0650451252"), true);
        assert_eq!(verify_iranian_national_id("4032152314"), true);
        assert_eq!(verify_iranian_national_id("0076229645"), true);
        assert_eq!(verify_iranian_national_id("4271467685"), true);
        assert_eq!(verify_iranian_national_id("0200203241"), true);
        assert_eq!(verify_iranian_national_id("068415941"), true);
        assert_eq!(verify_iranian_national_id("68415941"), true);
        assert_eq!(verify_iranian_national_id("787833770"), true);
    }

}