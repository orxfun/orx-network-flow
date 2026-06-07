use crate::problem::connectivity::{SpatialConnectivity, TemporalConnectivity};
use crate::transports::Transport;
use crate::{Problem, Variant};

#[derive(Default)]
pub struct Connectivity {
    pub spatial: SpatialConnectivity,
    pub temporal: TemporalConnectivity,
}

impl Connectivity {
    pub fn can_connect<V: Variant>(&self, p: &Problem<V>, i: Transport, j: Transport) -> bool {
        let [i, j] = [i, j].map(|t| p.transport_by_idx(t));
        let [a, b] = i.ori_des();
        let [b2, c] = j.ori_des();
        match b == b2 {
            false => false,
            true => match self.spatial.can_connect(p, a, b, c) {
                false => false,
                true => {
                    let at = i.destination().time();
                    let dt = j.origin().time();
                    self.temporal.can_connect(p, b, at, dt)
                }
            },
        }
    }
}
