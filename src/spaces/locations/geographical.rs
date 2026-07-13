use super::Location;

impl Location for Geographical {}

#[derive(derive_new::new)]
pub struct Geographical {
    pub lat: f64,
    pub lon: f64,
}
