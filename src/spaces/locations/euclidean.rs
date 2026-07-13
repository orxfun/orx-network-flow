use super::Location;

impl Location for Euclidean {}

#[derive(derive_new::new, Clone, Copy, Debug)]
pub struct Euclidean {
    pub x: f64,
    pub y: f64,
}

impl Euclidean {
    pub fn distance(self, other: Self) -> f64 {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        let sqr = x_diff * x_diff + y_diff * y_diff;
        sqr.sqrt()
    }
}
