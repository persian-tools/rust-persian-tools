mod errors;
mod place_by_iran_national_id;
mod places;

use self::{
    errors::PlaceByNationalIdError, place_by_iran_national_id::PlaceByNationalId,
    places::code_to_city,
};

/// 0499370899 -> تهران-شهرری
/// # Example:
/// ```
/// use rust_persian_tools::get_place_by_iran_national_id::get_place_by_iran_national_id;
/// let place = get_place_by_iran_national_id("0499370899").unwrap();
/// assert_eq!(place.get_city(), "شهرری");
/// assert_eq!(place.get_province(), "تهران");
/// ```
pub fn get_place_by_iran_national_id(
    inp: impl AsRef<str>,
) -> Result<PlaceByNationalId, PlaceByNationalIdError> {
    let inp = inp.as_ref();

    if inp.len() != 10 {
        return Err(PlaceByNationalIdError::TooShortNationalId(inp.len()));
    }

    let code: String = inp.chars().take(3).collect();

    if let Some(x) = code_to_city(&code) {
        Ok(x)
    } else {
        Err(PlaceByNationalIdError::NotFound)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            get_place_by_iran_national_id("0499370899"),
            Ok(PlaceByNationalId::new("شهرری", "تهران"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0790419904"),
            Ok(PlaceByNationalId::new("سبزوار", "خراسان رضوی"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0084575948"),
            Ok(PlaceByNationalId::new("تهران مرکزی", "تهران"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0060495219"),
            Ok(PlaceByNationalId::new("تهران مرکزی", "تهران"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0671658506"),
            Ok(PlaceByNationalId::new("بجنورد", "خراسان شمالی"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0671658506"),
            Ok(PlaceByNationalId::new("بجنورد", "خراسان شمالی"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0643005846"),
            Ok(PlaceByNationalId::new("بیرجند", "خراسان جنوبی"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0906582709"),
            Ok(PlaceByNationalId::new("کاشمر", "خراسان رضوی"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0451727304"),
            Ok(PlaceByNationalId::new("شمیران", "تهران"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0371359058"),
            Ok(PlaceByNationalId::new("قم", "قم"))
        );
        assert_eq!(
            get_place_by_iran_national_id("5049478618"),
            Ok(PlaceByNationalId::new("پارس آباد", "اردبیل"))
        );
        assert_eq!(
            get_place_by_iran_national_id("2110990147"),
            Ok(PlaceByNationalId::new("گرگان", "گلستان"))
        );
        assert_eq!(
            get_place_by_iran_national_id("0084545943"),
            Ok(PlaceByNationalId::new("تهران مرکزی", "تهران"))
        );
    }

    #[test]
    fn test_name_errors() {
        assert_eq!(
            get_place_by_iran_national_id("008454594"),
            Err(PlaceByNationalIdError::TooShortNationalId(9))
        );
        assert_eq!(
            get_place_by_iran_national_id("8881234567"),
            Err(PlaceByNationalIdError::NotFound)
        );
    }
}
