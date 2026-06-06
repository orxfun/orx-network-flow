use crate::{Variant, spaces::Geocode, std_utils::Set};

pub struct SpatialConnectivity<V: Variant> {
    /// It is not allowed to connect transport a->b with b->c
    /// if (a,b,c) is in the taboo_set.
    taboo_set: Set<(V::S, V::S, V::S)>,
    /// Geographical connectivity rules
    geographical_connectivity: Option<GeographicalConnectivity>,
}

pub struct GeographicalConnectivity {
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

impl Default for GeographicalConnectivity {
    fn default() -> Self {
        Self {
            near_ac_km: 500.0,
            far_via_b_km: 900.0,
            min_detour_ratio: 1.8,
            min_excess_km: 700.0,
            epsilon_ac_km: 50.0,
        }
    }
}

impl GeographicalConnectivity {
    fn can_connect(&self, a: Geocode, b: Geocode, c: Geocode) -> bool {
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

impl<V: Variant> SpatialConnectivity<V> {
    pub fn can_connect(&self, a: Geocode, b: Geocode, c: Geocode) -> bool {
        todo!()
    }
}
