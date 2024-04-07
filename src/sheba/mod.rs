use self::{
    bank_info::{get_bank_info, ShebaResult},
    checksum::sheba_iso7064_mod97,
};

pub mod bank_info;
mod checksum;
pub mod errors;
pub use errors::ShebaValidationError;

type E = errors::ShebaValidationError;

/// this function validates sheba code
/// this is valid: IR820540102680020817909002
/// # Example:
/// ```
/// use rust_persian_tools::sheba::is_sheba_valid;
/// assert_eq!(is_sheba_valid("IR550570022080013447370101"), Ok(()));
/// ```
pub fn is_sheba_valid(sheba: impl AsRef<str>) -> Result<(), E> {
    let mut sheba = sheba.as_ref().to_string();

    if sheba.chars().take(2).collect::<String>().to_uppercase() == "IR" {
        sheba = sheba.chars().skip(2).collect::<String>();
    }

    let len = sheba.chars().count();
    if len != 24 {
        return Err(E::InvalidLength(len));
    }

    if sheba.chars().any(|x| !x.is_ascii_digit()) {
        return Err(E::InvalidDigit);
    }

    let d1 = sheba.chars().skip(2).collect::<String>();
    let d2 = sheba.chars().take(2).collect::<String>();

    // thats where 1827 comes from:
    // 18 // 73 - 65 + 10 // 73 is accii code of I
    // 27 // 82 - 65 + 10 // 82 is accii code of R
    let new_str = d1 + "1827" + &d2;

    let remainder = sheba_iso7064_mod97(new_str)?;
    if remainder == 1 {
        Ok(())
    } else {
        Err(E::InvalidChecksum)
    }
}

/// get information about bank from sheba code
/// name, code, nickname, persian_name
/// # Example:
/// ```
/// use rust_persian_tools::sheba::get_sheba_info;
/// use rust_persian_tools::sheba::bank_info::ShebaResult;
/// let bank = get_sheba_info("IR790610000000700796858044").unwrap();
/// assert_eq!(bank.get_code(), "061");
/// assert_eq!(bank.get_name(), "City Bank");
/// assert_eq!(bank.get_nickname(), "shahr");
/// assert_eq!(bank.get_persian_name(), "بانک شهر");
/// ```
pub fn get_sheba_info(sheba: impl AsRef<str>) -> Result<ShebaResult, E> {
    let sheba = sheba.as_ref();

    is_sheba_valid(sheba)?;

    // this is safe length is checked at is_sheba_valid
    let code = sheba
        .chars()
        .skip(4)
        .take(3)
        .collect::<String>()
        .parse::<u32>()?;

    get_bank_info(code).ok_or(E::BankNotFound)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_sheba_valid_test() {
        assert_eq!(is_sheba_valid("IR820540102680020817909002"), Ok(()));
        assert_eq!(is_sheba_valid("IR550570022080013447370101"), Ok(()));

        assert_eq!(
            is_sheba_valid("IR01234567890123456789"),
            Err(E::InvalidLength(20))
        );
        assert_eq!(
            is_sheba_valid("IR012345678901234567890123456789"),
            Err(E::InvalidLength(30))
        );
        assert_eq!(
            is_sheba_valid("IR012345678901234567890123"),
            Err(E::InvalidChecksum)
        );
        assert_eq!(
            is_sheba_valid("012345678901234567890123"),
            Err(E::InvalidChecksum)
        );
    }

    #[test]
    fn get_sheba_info_test() {
        assert_eq!(
            get_sheba_info("IR790610000000700796858044"),
            Ok(ShebaResult {
                code: "061",
                name: "City Bank",
                nickname: "shahr",
                persian_name: "بانک شهر"
            })
        );
        assert_eq!(
            get_sheba_info("IR820540102680020817909002")
                .unwrap()
                .get_nickname(),
            "parsian"
        );
        assert_eq!(
            get_sheba_info("IR550570022080013447370101")
                .unwrap()
                .get_nickname(),
            "pasargad"
        );

        assert_eq!(
            get_sheba_info("IR012345678901234567890123"),
            Err(E::InvalidChecksum)
        );
        assert_eq!(
            get_sheba_info("IR012345678A01234567890123"),
            Err(E::InvalidDigit)
        );
    }
}
