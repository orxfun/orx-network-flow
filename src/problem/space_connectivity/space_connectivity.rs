use crate::Location;
use crate::problem::LocationConnectivity;
use crate::spaces::{Space, Spaces};
use crate::{Problem, Variant, utils::std_utils::Set};

#[derive(derive_new::new)]
pub struct SpaceConnectivityBuilder<'a, V: Variant> {
    spaces: &'a Spaces<V>,
    conn: &'a mut SpaceConnectivity<V>,
}

impl<V: Variant> SpaceConnectivityBuilder<'_, V> {
    pub fn ban_connection(&mut self, a: &V::S, b: &V::S, c: &V::S) {
        let [a, b, c] = [a, b, c].map(|s| self.spaces.get_ind_by_key(s).expect("invalid space"));
        self.conn.taboo_set.insert((a, b, c));
    }
}

pub struct SpaceConnectivity<V: Variant> {
    /// It is not allowed to connect transport a->b with b->c
    /// if (a,b,c) is in the taboo_set.
    taboo_set: Set<(Space, Space, Space)>,
    /// Location connectivity
    location_connectivity: <V::L as Location>::Connectivity,
}

impl<V: Variant> Default for SpaceConnectivity<V> {
    fn default() -> Self {
        Self {
            taboo_set: Default::default(),
            location_connectivity: Default::default(),
        }
    }
}

impl<V: Variant> SpaceConnectivity<V> {
    pub fn can_connect(&self, p: &Problem<V>, a: Space, b: Space, c: Space) -> bool {
        match self.taboo_set.contains(&(a, b, c)) {
            true => false,
            false => {
                let [a, b, c] = [a, b, c].map(|s| p.space_by_idx(s).location);
                self.location_connectivity.can_connect(a, b, c)
            }
        }
    }
}
