#[cfg(not(any(
    feature = "add-ordinal-suffix",
    feature = "commas",
    feature = "digits",
    feature = "find-capital-by-province",
    feature = "persian-chars",
    feature = "national-id",
    feature = "remove-ordinal-suffix",
    feature = "url-fix",
    feature = "verity-card-number",
    feature = "phone-number",
    feature = "number-to-words",
    feature = "get-bank-name-by-card-number",
    feature = "extract-card-number",
    feature = "get-place-by-iran-national-id",
)))]
compile_error!("No available Cargo feature is included");

#[cfg(feature = "add-ordinal-suffix")]
pub mod add_ordinal_suffix;

#[cfg(feature = "commas")]
pub mod commas;

#[cfg(feature = "digits")]
pub mod digits;

#[cfg(feature = "find-capital-by-province")]
pub mod find_capital_by_province;

#[cfg(feature = "persian-chars")]
pub mod persian_chars;

#[cfg(feature = "national-id")]
pub mod national_id;

#[cfg(feature = "remove-ordinal-suffix")]
pub mod remove_ordinal_suffix;

#[cfg(feature = "url-fix")]
pub mod url_fix;

#[cfg(feature = "verity-card-number")]
pub mod verity_card_number;

#[cfg(feature = "phone-number")]
pub mod phone_number;

#[cfg(feature = "bill")]
pub mod bill;

#[cfg(feature = "number-to-words")]
pub mod number_to_words;

#[cfg(feature = "get-bank-name-by-card-number")]
pub mod get_bank_name_by_card_number;

#[cfg(feature = "extract-card-number")]
pub mod extract_card_number;

#[cfg(feature = "time-ago")]
pub mod time_ago;

#[cfg(feature = "get-place-by-iran-national-id")]
pub mod get_place_by_iran_national_id;
