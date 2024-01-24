#[derive(PartialEq, Debug)]
pub struct PlateResult {
    pub info: PlateResultApi,
    pub is_valid: bool,
}

#[derive(PartialEq, Debug)]
pub enum PlateTypes {
    Car = 1,
    Motorcycle = 2,
}

#[derive(PartialEq, Debug)]
pub struct PlateResultCarDetailModel {
    pub first_two_digits: String,
    pub plate_character: Option<String>,
    pub next_three_digits: String,
    pub province_code: String,
}

#[derive(PartialEq, Debug)]
pub struct PlateResultMotorcycleDetailModel {
    pub digits: String,
    pub province_code: String,
}

#[derive(PartialEq, Debug)]
pub enum PlateResultApiDetails {
    CarDetail(PlateResultCarDetailModel),
    MotorcycleDetail(PlateResultMotorcycleDetailModel),
}

#[derive(PartialEq, Debug)]
pub struct PlateResultApi {
    pub template: String,
    pub province: Option<&'static str>,
    pub plate_type: PlateTypes,
    pub details: PlateResultApiDetails,
    pub category: Option<&'static str>,
}

#[derive(PartialEq, Debug)]
pub struct NormalizedPlate {
    pub numbers: String,
    pub char: Option<String>,
}
