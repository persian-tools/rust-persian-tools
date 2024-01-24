pub mod codes;
mod helpers;
pub mod types;

use codes::{car_dataset, category_dataset, motorcycle_dataset};
use helpers::{is_plate_number_valid, normalize_plate};
pub use types::{
    NormalizedPlate, PlateResult, PlateResultApi, PlateResultApiDetails, PlateResultCarDetailModel,
    PlateResultMotorcycleDetailModel, PlateTypes,
};

/// returns plate info and its validation
/// # Arguments:
/// * `plate` plate either as a string: '"12ب45147"'
/// # Examples:
/// ```
/// use rust_persian_tools::number_plate::{plate, PlateResultCarDetailModel, PlateTypes, PlateResultApiDetails, PlateResultApi};
/// let plate_result = plate("1214547ب");
/// let template: String = format!("12{}145{}47","ب","ایران");
/// let details = PlateResultCarDetailModel {
///     first_two_digits: "12".to_owned(),
///     plate_character: Some("ب".to_owned()),
///     next_three_digits: "145".to_owned(),
///     province_code: "47".to_owned(),
/// };
/// let plate_info = PlateResultApi {
///     plate_type: PlateTypes::Car,
///     template,
///     province: Some("مرکزی"),
///     category: Some("شخصی"),
///     details:PlateResultApiDetails::CarDetail(details),
/// };
/// assert!(plate_result.is_valid);
/// assert_eq!(plate_result.info, plate_info);
/// ```
pub fn plate(plate: impl Into<String>) -> PlateResult {
    let normalized_plate = normalize_plate(plate);
    let info = get_plate_info(&normalized_plate);
    let is_valid = is_plate_valid(&info, &normalized_plate.numbers);

    PlateResult { info, is_valid }
}

/// returns plate info based on plate type
/// # Examples:
/// ```
/// use rust_persian_tools::number_plate::plate;
/// let plate_result = plate("12ب45147");
/// assert!(plate_result.is_valid);
/// ```
pub fn get_plate_info(plate: &NormalizedPlate) -> PlateResultApi {
    match plate.numbers.len() {
        7 => get_car_info(plate),
        8 => get_motorcycle_info(plate),
        _ => panic!("plate number must be 7 or 8 digits long"),
    }
}

fn is_plate_valid(plate_info: &PlateResultApi, plate_number: &str) -> bool {
    if !is_plate_number_valid(plate_number) {
        return false;
    }

    if let PlateTypes::Car = plate_info.plate_type {
        if plate_info.category.is_none() {
            return false;
        }
    }

    if plate_info.province.is_none() {
        return false;
    }

    true
}

fn get_car_info(plate: &NormalizedPlate) -> PlateResultApi {
    let province_code: u32 = plate.numbers[5..7].parse().expect("Invalid province code");
    let plate_type = PlateTypes::Car;
    let template = format!(
        "{}{}{}ایران{}",
        &plate.numbers[0..2],
        &plate.char.as_ref().unwrap_or(&"".to_string()),
        &plate.numbers[2..5],
        province_code
    );

    let province = car_dataset().get(&province_code).cloned();
    let category = plate
        .char
        .as_ref()
        .and_then(|c| category_dataset().get(c.as_str()).cloned());

    let details = PlateResultCarDetailModel {
        first_two_digits: plate.numbers[0..2].to_string(),
        plate_character: plate.char.clone(),
        next_three_digits: plate.numbers[2..5].to_string(),
        province_code: province_code.to_string(),
    };

    PlateResultApi {
        plate_type,
        template,
        details: PlateResultApiDetails::CarDetail(details),
        province,
        category,
    }
}

fn get_motorcycle_info(plate: &NormalizedPlate) -> PlateResultApi {
    let province_code: u32 = plate.numbers[0..3].parse().expect("Invalid province code");
    let plate_type = PlateTypes::Motorcycle;
    let template = format!("{}-{}", &plate.numbers[0..3], &plate.numbers[3..]);

    let province = motorcycle_dataset().get(&province_code).cloned();

    let details = PlateResultMotorcycleDetailModel {
        digits: plate.numbers[3..].to_string(),
        province_code: province_code.to_string(),
    };

    PlateResultApi {
        plate_type,
        template,
        province,
        details: PlateResultApiDetails::MotorcycleDetail(details),
        category: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_car_plate() {
        let plate_result = plate("1214547ب");
        let template: String = format!("12{}145{}47", "ب", "ایران");
        let details = PlateResultCarDetailModel {
            first_two_digits: "12".to_owned(),
            plate_character: Some("ب".to_owned()),
            next_three_digits: "145".to_owned(),
            province_code: "47".to_owned(),
        };
        let plate_info = PlateResultApi {
            plate_type: PlateTypes::Car,
            template,
            province: Some("مرکزی"),
            category: Some("شخصی"),
            details: PlateResultApiDetails::CarDetail(details),
        };
        assert!(plate_result.is_valid);
        assert_eq!(plate_result.info, plate_info);
    }

    #[test]
    fn should_return_none_for_fake_province_and_char_in_info_and_invalid() {
        let plate_result = plate("1214501"); //fake province code (01) && no char
        let template: String = format!("12145{}1", "ایران");
        let details = PlateResultCarDetailModel {
            first_two_digits: "12".to_owned(),
            plate_character: None,
            next_three_digits: "145".to_owned(),
            province_code: "1".to_owned(),
        };
        let plate_info = PlateResultApi {
            plate_type: PlateTypes::Car,
            template,
            province: None,
            category: None,
            details: PlateResultApiDetails::CarDetail(details),
        };
        assert!(!plate_result.is_valid);
        assert_eq!(plate_result.info, plate_info);
    }

    #[test]
    fn should_parse_motorcycle_plate() {
        let plate_result = plate("12145478");
        let details = PlateResultMotorcycleDetailModel {
            digits: "45478".to_owned(),
            province_code: "121".to_owned(),
        };

        let plate_info = PlateResultApi {
            plate_type: PlateTypes::Motorcycle,
            template: "121-45478".to_owned(),
            province: Some("مرکز تهران"),
            category: None,
            details: PlateResultApiDetails::MotorcycleDetail(details),
        };
        assert!(plate_result.is_valid);
        assert_eq!(plate_result.info, plate_info);
    }

    #[test]
    fn should_return_invalid_for_false_car_charachter() {
        let plate_result = plate("1214547f");
        assert!(!plate_result.is_valid);
    }
}
