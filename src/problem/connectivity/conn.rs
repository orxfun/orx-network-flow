use crate::problem::connectivity::{spatial::SpatialConnectivity, temporal::TemporalConnectivity};

#[derive(Default)]
pub struct Connectivity {
    pub spatial: SpatialConnectivity,
    pub temporal: TemporalConnectivity,
}
