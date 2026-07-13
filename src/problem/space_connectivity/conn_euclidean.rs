use crate::Euclidean;
use crate::problem::space_connectivity::LocationConnectivity;

pub struct ConnectivityEuclidean {
    /// A and C are considered close if direct distance is less than or equal to this threshold.
    pub near_ac: f64,
    /// B is considered far from both A and C if both legs exceed this threshold.
    pub far_via_b: f64,
    /// Relative detour threshold: (A-B + B-C) / max(A-C, epsilon_ac).
    pub min_detour_ratio: f64,
    /// Absolute detour threshold: (A-B + B-C) - (A-C).
    pub min_excess: f64,
    /// Lower bound on direct distance denominator to avoid instability around very short A-C.
    pub epsilon_ac: f64,
}

impl LocationConnectivity for ConnectivityEuclidean {
    type L = Euclidean;

    fn can_connect(&self, a: Self::L, b: Self::L, c: Self::L) -> bool {
        let d_ab = a.distance(b);
        let d_bc = b.distance(c);
        let d_ac = a.distance(c);

        let path = d_ab + d_bc;
        let detour_ratio = path / d_ac.max(self.epsilon_ac);
        let excess = path - d_ac;

        let ac_is_near = d_ac <= self.near_ac;
        let b_is_far_from_both = d_ab >= self.far_via_b && d_bc >= self.far_via_b;
        let detour_is_large = detour_ratio >= self.min_detour_ratio && excess >= self.min_excess;

        let banned = ac_is_near && b_is_far_from_both && detour_is_large;
        !banned
    }
}

impl Default for ConnectivityEuclidean {
    fn default() -> Self {
        Self {
            near_ac: 500.0,
            far_via_b: 900.0,
            min_detour_ratio: 1.8,
            min_excess: 700.0,
            epsilon_ac: 50.0,
        }
    }
}
