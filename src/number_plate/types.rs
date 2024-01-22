#[derive(Debug)]
pub struct PlateResult {
    pub info: PlateResultApi,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct PlateApi {
    pub number: String,
    pub char: Option<String>,
}

#[derive(Debug)]
pub enum PlateTypes {
    Car = 1,
    Motorcycle = 2,
}

#[derive(Debug)]
pub struct PlateResultDetailModel {
    pub first_two_digits: String,
    pub plate_character: Option<String>,
    pub next_three_digits: String,
    pub province_code: String,
}

#[derive(Debug)]
pub struct PlateResultMotorcycleDetailModel {
    pub digits: String,
    pub province_code: String,
}

#[derive(Debug)]
pub enum PlateResultApiDetails {
    CarDetail(PlateResultDetailModel),
    MotorcycleDetail(PlateResultMotorcycleDetailModel),
}

#[derive(Debug)]
pub struct PlateResultApi {
    pub template: String,
    pub province: Option<&'static str>,
    pub plate_type: PlateTypes,
    pub details: PlateResultApiDetails,
    pub category: Option<&'static str>,
}

#[derive(Debug)]
pub enum PlateOptions {
    Str(String),
    Api(PlateApi),
}

#[derive(Debug)]
pub struct NormalizedPlate {
    pub numbers: String,
    pub char: Option<String>,
}
