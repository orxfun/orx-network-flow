use crate::spaces::Spaces;
use crate::spaces::{Coordinate, Geocode, Space};
use crate::{Problem, Variant, utils::std_utils::Set};

#[derive(derive_new::new)]
pub struct SpaceConnectivityBuilder<'a, V: Variant> {
    spaces: &'a Spaces<V>,
    conn: &'a mut SpaceConnectivity,
}

#[derive(Default)]
pub struct SpaceConnectivity<V: {
    /// It is not allowed to connect transport a->b with b->c
    /// if (a,b,c) is in the taboo_set.
    taboo_set: Set<(Space, Space, Space)>,
    /// Geographical connectivity rules
    geographical_connectivity: Option<GeographicalConnectivity>,
    /// Euclidean connectivity rules
    euclidean_connectivity: Option<EuclideanConnectivity>,
}
wip