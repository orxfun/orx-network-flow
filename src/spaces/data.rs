#[derive(derive_new::new)]
pub struct SpaceData {
    pub location: Location,
}

#[derive(Clone, Copy)]
pub enum Location {
    Basic,
    Euclidean(Coordinate),
    Geographic(Geocode),
}

impl Location {
    pub fn distance(self, other: Self) -> f64 {
        match (self, other) {
            (Self::Basic, Self::Basic) => 0.0,
            (Self::Euclidean(x), Self::Euclidean(y)) => x.distance(y),
            (Self::Geographic(x), Self::Geographic(y)) => x.distance_km(y),
            _ => unreachable!("location kinds are consistent by problem construction"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub fn distance(self, other: Self) -> f64 {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        let sqr = x_diff * x_diff + y_diff * y_diff;
        sqr.sqrt()
    }
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
