pub struct SpaceData {
    geocode: Option<Geocode>,
}

#[derive(Clone, Copy)]
pub struct Geocode {
    pub lat: f64,
    pub lon: f64,
}

impl Geocode {
    pub fn distance_km(self, other: Self) -> f64 {
        // Mean Earth radius in kilometers (IUGG mean radius).
        const EARTH_RADIUS_KM: f64 = 6_371.008_8;

        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();
        let diff_lat = (other.lat - self.lat).to_radians();
        let diff_lon = (other.lon - self.lon).to_radians();

        let sin_diff_lat = (diff_lat * 0.5).sin();
        let sin_d_lon = (diff_lon * 0.5).sin();

        let a = sin_diff_lat * sin_diff_lat + lat1.cos() * lat2.cos() * sin_d_lon * sin_d_lon;
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS_KM * c
    }
}
