use super::types::{NormalizedPlate, PlateOptions};

pub fn normalize_plate(plate: PlateOptions) -> NormalizedPlate {
    let (char, plate_numbers) = match plate {
        PlateOptions::Str(s) => {
            let chars: String = s.chars().filter(|c| c.is_alphabetic()).collect();
            let nums: String = s.chars().filter(|c| c.is_numeric()).collect();
            (Some(chars), nums)
        }
        PlateOptions::Api(api) => (
            api.char,
            api.number.chars().filter(|c| c.is_numeric()).collect(),
        ),
    };

    NormalizedPlate {
        numbers: plate_numbers,
        char,
    }
}

pub fn is_plate_number_valid(numbers: &str) -> bool {
    match numbers.parse::<u64>() {
        Ok(_) => numbers.chars().take(numbers.len() - 1).all(|c| c != '0'),
        Err(_) => false,
    }
}
