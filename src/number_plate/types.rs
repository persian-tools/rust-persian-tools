#[derive(PartialEq, Debug)]
pub enum PlateTypes {
    Car = 1,
    Motorcycle = 2,
}

#[derive(PartialEq, Debug)]
pub struct CarPlateDetail {
    pub first_two_digits: String,
    pub plate_character: String,
    pub next_three_digits: String,
    pub province_code: String,
}

#[derive(PartialEq, Debug)]
pub struct MotorcyclePlateDetail {
    pub digits: String,
    pub province_code: String,
}

#[derive(PartialEq, Debug)]
pub enum PlateResultDetails {
    CarDetail(CarPlateDetail),
    MotorcycleDetail(MotorcyclePlateDetail),
}

#[derive(PartialEq, Debug)]
pub struct Plate {
    pub template: String,
    pub province: String,
    pub plate_type: PlateTypes,
    pub details: PlateResultDetails,
    pub category: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct NormalizedPlate {
    pub numbers: String,
    pub char: Option<String>,
}
