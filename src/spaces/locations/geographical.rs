use super::Location;

impl Location for Geographical {}

pub struct Geographical {
    pub lat: f64,
    pub lon: f64,
}
