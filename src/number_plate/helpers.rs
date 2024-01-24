use std::ops::Not;

use super::types::NormalizedPlate;

pub fn normalize_plate(plate: impl Into<String>) -> NormalizedPlate {
    let plate = plate.into();
    let char: String = plate.chars().filter(|c| c.is_alphabetic()).collect();
    let nums: String = plate.chars().filter(|c| c.is_numeric()).collect();
    NormalizedPlate {
        numbers: nums,
        char: char.is_empty().not().then_some(char),
    }
}

pub fn is_plate_number_valid(numbers: &str) -> bool {
    match numbers.parse::<u64>() {
        Ok(_) => numbers.chars().take(numbers.len() - 1).all(|c| c != '0'),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    mod normalize_plate {
        use crate::number_plate::types::NormalizedPlate;

        use super::super::normalize_plate;
        #[test]
        fn should_parse_to_normalized_plate() {
            let plate_object = normalize_plate("1234567الف");
            assert_eq!(
                plate_object,
                NormalizedPlate {
                    numbers: "1234567".to_owned(),
                    char: Some("الف".to_owned())
                }
            );
        }

        #[test]
        fn should_parse_plate_with_no_alphabet_char() {
            let plate_object = normalize_plate("1234567");
            assert_eq!(
                plate_object,
                NormalizedPlate {
                    numbers: "1234567".to_owned(),
                    char: None
                }
            );
        }
    }
    mod is_plate_number {
        use super::super::is_plate_number_valid;

        #[test]
        fn should_validate_car_plate() {
            assert!(is_plate_number_valid("1234564")); // Car plate
            assert!(is_plate_number_valid("12345678")); // Motorcycle plate
            assert!(is_plate_number_valid("12345670"));

            assert!(!is_plate_number_valid("1230567"));
            assert!(!is_plate_number_valid("12305678"));
            assert!(!is_plate_number_valid("1ی23456"));
            assert!(!is_plate_number_valid("1234f560"));
            assert!(!is_plate_number_valid("123450d0"));
        }

        #[test]
        fn should_validate_motorcycle_plate() {
            assert!(is_plate_number_valid("12345678"));
        }

        #[test]
        fn should_validate_zero_as_last_number() {
            assert!(is_plate_number_valid("12345670"));
            assert!(is_plate_number_valid("1234560"));
        }

        #[test]
        fn should_return_false_for_invalid_cases() {
            assert!(!is_plate_number_valid("1230567"));
            assert!(!is_plate_number_valid("12305678"));
            assert!(!is_plate_number_valid("1ی23456"));
            assert!(!is_plate_number_valid("1234f560"));
            assert!(!is_plate_number_valid("123450d0"));
        }
    }
}
