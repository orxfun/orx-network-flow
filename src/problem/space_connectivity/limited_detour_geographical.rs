use crate::Geographical;
use crate::problem::space_connectivity::LocationConnectivity;

pub struct LimitedDetourGeographical {
    /// A and C are considered close if direct distance is less than or equal to this threshold.
    pub near_ac_km: f64,
    /// B is considered far from both A and C if both legs exceed this threshold.
    pub far_via_b_km: f64,
    /// Relative detour threshold: (A-B + B-C) / max(A-C, epsilon_ac_km).
    pub min_detour_ratio: f64,
    /// Absolute detour threshold: (A-B + B-C) - (A-C).
    pub min_excess_km: f64,
    /// Lower bound on direct distance denominator to avoid instability around very short A-C.
    pub epsilon_ac_km: f64,
}

impl LocationConnectivity for LimitedDetourGeographical {
    type L = Geographical;

    fn can_connect(&self, a: Self::L, b: Self::L, c: Self::L) -> bool {
        let d_ab = a.distance_km(b);
        let d_bc = b.distance_km(c);
        let d_ac = a.distance_km(c);

        let path_km = d_ab + d_bc;
        let detour_ratio = path_km / d_ac.max(self.epsilon_ac_km);
        let excess_km = path_km - d_ac;

        let ac_is_near = d_ac <= self.near_ac_km;
        let b_is_far_from_both = d_ab >= self.far_via_b_km && d_bc >= self.far_via_b_km;
        let detour_is_large =
            detour_ratio >= self.min_detour_ratio && excess_km >= self.min_excess_km;

        let banned = ac_is_near && b_is_far_from_both && detour_is_large;
        !banned
    }
}
