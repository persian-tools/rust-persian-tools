use std::str::FromStr;

use crate::constants::*;
use rust_persian_tools::*;

pub fn add_commas(s: &str) -> String {
    commas::add_commas::add_commas(s)
}
pub fn remove_commas(s: &str) -> String {
    commas::remove_commas::remove_commas(s)
}
// ---
pub fn add_ordinal_suffix(s: &str) -> String {
    add_ordinal_suffix::add_ordinal_suffix(s)
}
// ---
pub fn remove_ordinal_suffix(s: &str) -> String {
    remove_ordinal_suffix::remove_ordinal_suffix(s)
}
// --- returns true|false
pub fn has_arabic(s: &str) -> String {
    arabic_chars::has_arabic(s).to_string()
}
// --- returns true|false
pub fn is_arabic(s: &str) -> String {
    arabic_chars::is_arabic(s).to_string()
}
pub fn to_arabic(s: &str) -> String {
    arabic_chars::to_arabic_chars(s)
}
// --- invalid | <bill_type>
pub fn get_bill_type(s: &str) -> String {
    match bill::Bill::from_str(s) {
        Ok(x) => format!("{:?}", x.get_bill_type()), // make a way to different error from ok
        Err(_) => INVALID.to_string(),
    }
}

// ---
pub fn fa_to_en(s: &str) -> String {
    digits::fa_to_en(s)
}
pub fn en_to_fa(s: &str) -> String {
    digits::en_to_fa(s)
}
pub fn en_to_ar(s: &str) -> String {
    digits::en_to_ar(s)
}
pub fn ar_to_en(s: &str) -> String {
    digits::ar_to_en(s)
}
pub fn fa_to_ar(s: &str) -> String {
    digits::fa_to_ar(s)
}
pub fn ar_to_fa(s: &str) -> String {
    digits::ar_to_fa(s)
}
// --- card numbers separated with commas
pub fn extract_card_number(s: &str) -> String {
    extract_card_number::extract_card_number(s)
        .iter()
        .map(|x| x.get_base())
        .collect::<Vec<&str>>()
        .join(SEPARATOR)
}
// --- not-found | <capital>
pub fn find_capital_by_province(s: &str) -> String {
    find_capital_by_province::find_capital_by_province(s)
        .unwrap_or(NOT_FOUND)
        .to_string()
}
// --- not-found
pub fn get_bank_name_by_card_number(s: &str) -> String {
    match get_bank_name_by_card_number::get_bank_name_by_card_number(s) {
        Ok(x) => x.to_string(),
        Err(e) => match e {
            get_bank_name_by_card_number::BankNameByCardNumberError::NotFound(_) => {
                NOT_FOUND.to_string()
            }
            get_bank_name_by_card_number::BankNameByCardNumberError::TooShort(_, _) => {
                "err: too short".to_string()
            }
        },
    }
}
// --- get_place_by_iran_national_id is separated to two functions
pub fn get_city_by_iran_national_id(s: &str) -> String {
    match get_place_by_iran_national_id::get_place_by_iran_national_id(s) {
        Ok(x) => x.get_city().to_string(),
        Err(_) => ERROR.to_string(),
    }
}
pub fn get_province_by_iran_national_id(s: &str) -> String {
    match get_place_by_iran_national_id::get_place_by_iran_national_id(s) {
        Ok(x) => x.get_province().to_string(),
        Err(_) => ERROR.to_string(),
    }
}

// ---
pub fn remove_half_space(s: &str) -> String {
    half_space::remove_half_space(s)
}
pub fn add_half_space(s: &str) -> String {
    half_space::add_half_space(s)
}

// ---
pub fn verify_iranian_legal_id(s: &str) -> String {
    legal_id::verify_iranian_legal_id(s).is_ok().to_string()
}
// ---
pub fn verify_iranian_national_id(s: &str) -> String {
    national_id::verify_iranian_national_id(s)
        .is_ok()
        .to_string()
}
// ---
pub fn get_plate_type(s: &str) -> String {
    match rust_persian_tools::number_plate::get_plate_info(s) {
        Ok(plate) => {
            format!("{:?}", plate.plate_type)
        }
        Err(_e) => ERROR.to_string(),
    }
}
pub fn get_plate_province(s: &str) -> String {
    match rust_persian_tools::number_plate::get_plate_info(s) {
        Ok(plate) => plate.province,
        Err(_e) => ERROR.to_string(),
    }
}
pub fn get_plate_category(s: &str) -> String {
    match rust_persian_tools::number_plate::get_plate_info(s) {
        Ok(plate) => {
            if let Some(category) = plate.category {
                category
            } else {
                NOT_FOUND.to_string()
            }
        }
        Err(_e) => ERROR.to_string(),
    }
}
// ---
pub fn number_to_words(s: &str) -> String {
    number_to_words::number_to_words_str(s).unwrap_or(ERROR.to_string())
}
// ---
pub fn has_persian(s: &str) -> String {
    persian_chars::has_persian(s, false).to_string()
}
pub fn is_persian(s: &str) -> String {
    persian_chars::is_persian(s, false).to_string()
}
pub fn to_persian_chars(s: &str) -> String {
    persian_chars::to_persian_chars(s)
}
// ---
pub fn is_phone_valid(s: &str) -> String {
    phone_number::is_phone_valid(s).is_ok().to_string()
}
pub fn get_operator_prefix(s: &str) -> String {
    phone_number::get_operator_prefix(s)
        .unwrap_or(INVALID)
        .to_string()
}
pub fn get_phone_operator(s: &str) -> String {
    match phone_number::operators::get_phone_details(s) {
        Ok(x) => format!("{:?}", x.operator()),
        Err(_) => INVALID.to_string(),
    }
}
pub fn get_phone_province(s: &str) -> String {
    match phone_number::operators::get_phone_details(s) {
        Ok(x) => format!("{:?}", x.base()),
        Err(_) => INVALID.to_string(),
    }
}
// ---
pub fn is_sheba_valid(s: &str) -> String {
    sheba::is_sheba_valid(s).is_ok().to_string()
}
pub fn sheba_to_bank_name(s: &str) -> String {
    match sheba::get_sheba_info(s) {
        Ok(x) => x.get_name().to_string(),
        Err(_e) => INVALID.to_string(),
    }
}
pub fn sheba_to_persian_bank_name(s: &str) -> String {
    match sheba::get_sheba_info(s) {
        Ok(x) => x.get_persian_name().to_string(),
        Err(_e) => INVALID.to_string(),
    }
}
// ---
pub fn time_diff(s: &str) -> String {
    match rust_persian_tools::time_diff::time_diff_now(s) {
        Ok(time_diff) => time_diff.long_form(),
        Err(_e) => ERROR.to_string(),
    }
}
// --
pub fn url_fix(s: &str) -> String {
    url_fix::url_fix(s, None).unwrap_or(ERROR.to_string())
}
// --
pub fn verify_card_number(s: &str) -> String {
    verity_card_number::verify_card_number(s)
        .is_ok()
        .to_string()
}
// ---
pub fn words_to_number(s: &str) -> String {
    words_to_number::words_to_number_str(
        s,
        &rust_persian_tools::words_to_number::Options::default(),
    )
    .unwrap_or(ERROR.to_string())
}
