mod codes;
mod helpers;
mod types;

use codes::{car_dataset, category_dataset, motorcycle_dataset};
use helpers::{is_plate_number_valid, normalize_plate};
use types::{
    NormalizedPlate, PlateOptions, PlateResult, PlateResultApi, PlateResultApiDetails,
    PlateResultDetailModel, PlateResultMotorcycleDetailModel, PlateTypes,
};

// Main function to get plate info and validation
pub fn plate(plate: PlateOptions) -> PlateResult {
    let normalized_plate = normalize_plate(plate);
    let info = get_plate_info(&normalized_plate);
    let is_valid = is_plate_valid(&info, &normalized_plate.numbers);

    PlateResult { info, is_valid }
}

pub fn get_plate_info(plate: &NormalizedPlate) -> PlateResultApi {
    let get_info = get_plate_handler(plate);
    get_info(plate)
}

pub fn is_plate_valid(plate_info: &PlateResultApi, plate_number: &str) -> bool {
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

pub fn get_plate_handler(plate: &NormalizedPlate) -> fn(&NormalizedPlate) -> PlateResultApi {
    if plate.numbers.len() == 7 {
        car_handler
    } else if plate.numbers.len() == 8 {
        motorcycle_handler
    } else {
        panic!("a Plate must be 7 or 8 digits long");
    }
}

pub fn car_handler(plate: &NormalizedPlate) -> PlateResultApi {
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

    let details = PlateResultDetailModel {
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

pub fn motorcycle_handler(plate: &NormalizedPlate) -> PlateResultApi {
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
