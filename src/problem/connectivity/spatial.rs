use crate::spaces::{Coordinate, Geocode, Location, Space};
use crate::{Problem, Variant, std_utils::Set};

#[derive(derive_new::new)]
pub struct SpatialConnectivityBuilder<'a, V: Variant> {
    p: &'a Problem<V>,
    conn: &'a mut SpatialConnectivity,
}

impl<'a, V: Variant> SpatialConnectivityBuilder<'a, V> {
    pub fn ban_connection(self, a: &V::S, b: &V::S, c: &V::S) -> Self {
        let [a, b, c] = [a, b, c].map(|s| self.p.space_idx(s).expect("invalid space"));
        self.conn.taboo_set.insert((a, b, c));
        self
    }

    pub fn with_geographical_connectivity(self, settings: GeographicalConnectivity) -> Self {
        self.conn.geographical_connectivity = Some(settings);
        self
    }

    pub fn with_euclidean_connectivity(self, settings: EuclideanConnectivity) -> Self {
        self.conn.euclidean_connectivity = Some(settings);
        self
    }

    pub fn can_connect(&self, a: &V::S, b: &V::S, c: &V::S) -> bool {
        let [a, b, c] = [a, b, c].map(|s| self.p.space_idx(s).expect("invalid space"));
        self.conn.can_connect(self.p, a, b, c)
    }
}

#[derive(Default)]
pub struct SpatialConnectivity {
    /// It is not allowed to connect transport a->b with b->c
    /// if (a,b,c) is in the taboo_set.
    taboo_set: Set<(Space, Space, Space)>,
    /// Geographical connectivity rules
    geographical_connectivity: Option<GeographicalConnectivity>,
    /// Euclidean connectivity rules
    euclidean_connectivity: Option<EuclideanConnectivity>,
}

impl SpatialConnectivity {
    pub fn new() -> Self {
        Self {
            taboo_set: Default::default(),
            geographical_connectivity: None,
            euclidean_connectivity: None,
        }
    }

    pub fn can_connect<V: Variant>(&self, p: &Problem<V>, a: Space, b: Space, c: Space) -> bool {
        match self.taboo_set.contains(&(a, b, c)) {
            true => false,
            false => {
                let [a, b, c] = [a, b, c].map(|s| p.space_by_idx(s).location);
                match (a, b, c) {
                    (Location::Basic, Location::Basic, Location::Basic) => true,
                    (Location::Euclidean(a), Location::Euclidean(b), Location::Euclidean(c)) => {
                        match &self.euclidean_connectivity {
                            None => true,
                            Some(conn) => conn.can_connect(a, b, c),
                        }
                    }
                    (Location::Geographic(a), Location::Geographic(b), Location::Geographic(c)) => {
                        match &self.geographical_connectivity {
                            None => true,
                            Some(conn) => conn.can_connect(a, b, c),
                        }
                    }
                    _ => unreachable!("consistent locations by construction"),
                }
            }
        }
    }
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

pub struct EuclideanConnectivity {
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

impl EuclideanConnectivity {
    pub fn can_connect(&self, a: Coordinate, b: Coordinate, c: Coordinate) -> bool {
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
