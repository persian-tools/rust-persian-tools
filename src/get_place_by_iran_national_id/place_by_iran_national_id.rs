#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct PlaceByNationalId {
    city: &'static str,
    province: &'static str,
}

impl PlaceByNationalId {
    pub fn new(city: &'static str, province: &'static str) -> Self {
        PlaceByNationalId { city, province }
    }

    pub fn get_province(&self) -> &'static str {
        self.province
    }

    pub fn get_city(&self) -> &'static str {
        self.city
    }
}
