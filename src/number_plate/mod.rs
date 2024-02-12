pub mod codes;
pub mod errors;
pub mod types;

use codes::{car_dataset, category_dataset, motorcycle_dataset};
pub use types::{CarPlateDetail, MotorcyclePlateDetail, NormalizedPlate, Plate, PlateTypes};

use self::{errors::PlateNumberError, types::PlateResultDetails};

/// returns plate info based on plate type
pub fn get_plate_info(plate: impl AsRef<str>) -> Result<Plate, PlateNumberError> {
    let plate = plate.as_ref();

    let (numbers, char): (String, String) = plate.chars().partition(|c| c.is_numeric());

    // if its a car plate and it has no charachter
    // if its a motocyle plate and it has charachters
    if (numbers.len() == 7 && char.len() != 1) && (numbers.len() == 8 && !char.is_empty()) {
        return Err(PlateNumberError::InvalidPlateCharacterLength);
    }

    match numbers.len() {
        7 => get_car_info(numbers, char),
        8 => get_motorcycle_info(numbers),
        _ => Err(PlateNumberError::InvalidPlateDigitLength),
    }
}

fn get_car_info(numbers: String, char: String) -> Result<Plate, PlateNumberError> {
    // The `unwrap()` call is safe because it's contingent upon a precondition: the `numbers`
    // length argument must be exactly 7, ensuring the presence of a valid range to unwrap.
    let province_code: u32 = numbers[5..7].parse().unwrap();
    let plate_type = PlateTypes::Car;
    let template = format!(
        "{}{}{}ایران{}",
        &numbers[0..2],
        &char,
        &numbers[2..5],
        province_code
    );

    let province = car_dataset().get(&province_code).cloned().map_or_else(
        || {
            Err(PlateNumberError::MotorcycleProvinceNotFound(
                numbers[5..7].to_string(),
            ))
        },
        Ok,
    )?;

    let category = category_dataset().get(&*char).cloned().map_or_else(
        || Err(PlateNumberError::InvalidPlateCharacter(char.clone())),
        Ok,
    )?;

    let details = CarPlateDetail {
        first_two_digits: numbers[0..2].to_string(),
        plate_character: char,
        next_three_digits: numbers[2..5].to_string(),
        province_code: province_code.to_string(),
    };

    Ok(Plate {
        plate_type,
        template,
        details: PlateResultDetails::CarDetail(details),
        province: province.to_string(),
        category: Some(category.to_string()),
    })
}

fn get_motorcycle_info(numbers: String) -> Result<Plate, PlateNumberError> {
    // The `unwrap()` call is safe because it's contingent upon a precondition: the `numbers`
    // length argument must be exactly 8, ensuring the presence of a valid range to unwrap.
    let province_code: u32 = numbers[0..3].parse().unwrap();
    let plate_type = PlateTypes::Motorcycle;
    let template = format!("{}-{}", &numbers[0..3], &numbers[3..]);

    let province = motorcycle_dataset()
        .get(&province_code)
        .cloned()
        .map_or_else(
            || {
                Err(PlateNumberError::MotorcycleProvinceNotFound(
                    numbers[0..3].to_string(),
                ))
            },
            Ok,
        )?;

    let details = MotorcyclePlateDetail {
        digits: numbers[3..].to_string(),
        province_code: province_code.to_string(),
    };

    Ok(Plate {
        plate_type,
        template,
        province: province.to_string(),
        details: PlateResultDetails::MotorcycleDetail(details),
        category: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_car_plate() {
        let plate_result = get_plate_info("1214547ب").unwrap();
        let template: String = format!("12{}145{}47", "ب", "ایران");
        let details = CarPlateDetail {
            first_two_digits: "12".to_owned(),
            plate_character: "ب".to_owned(),
            next_three_digits: "145".to_owned(),
            province_code: "47".to_owned(),
        };
        let plate_info = Plate {
            plate_type: PlateTypes::Car,
            template,
            province: "مرکزی".to_string(),
            category: Some("شخصی".to_string()),
            details: PlateResultDetails::CarDetail(details),
        };
        assert_eq!(plate_result, plate_info);
    }

    #[test]
    fn should_return_error() {
        let plate_result = get_plate_info("1214501"); //fake province code (01) && no char
        assert!(plate_result.is_err());
    }

    #[test]
    fn should_parse_motorcycle_plate() {
        let plate_result = get_plate_info("12145478").unwrap();

        let details = MotorcyclePlateDetail {
            digits: "45478".to_owned(),
            province_code: "121".to_owned(),
        };

        let plate_info = Plate {
            plate_type: PlateTypes::Motorcycle,
            template: "121-45478".to_owned(),
            province: "مرکز تهران".to_string(),
            category: None,
            details: PlateResultDetails::MotorcycleDetail(details),
        };
        assert_eq!(plate_result, plate_info);
    }

    #[test]
    fn should_return_invalid_for_false_car_charachter() {
        let plate_result = get_plate_info("1214547f");
        assert!(plate_result.is_err());
    }
}
