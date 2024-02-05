#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub enum PlateTypes {
    Car = 1,
    Motorcycle = 2,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct CarPlateDetail {
    pub first_two_digits: String,
    pub plate_character: String,
    pub next_three_digits: String,
    pub province_code: String,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct MotorcyclePlateDetail {
    pub digits: String,
    pub province_code: String,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub enum PlateResultDetails {
    CarDetail(CarPlateDetail),
    MotorcycleDetail(MotorcyclePlateDetail),
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Plate {
    pub template: String,
    pub province: String,
    pub plate_type: PlateTypes,
    pub details: PlateResultDetails,
    pub category: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct NormalizedPlate {
    pub numbers: String,
    pub char: Option<String>,
}
