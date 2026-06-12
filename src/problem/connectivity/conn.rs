use crate::problem::connectivity::{SpatialConnectivity, TemporalConnectivity};
use crate::spaces::Space;
use crate::transports::Transport;
use crate::{Problem, Variant};

#[derive(Default)]
pub struct Connectivity {
    pub spatial: SpatialConnectivity,
    pub temporal: TemporalConnectivity,
}

impl Connectivity {
    pub fn can_connect_spatially<V: Variant>(&self, p: &Problem<V>, [a, b, c]: [Space; 3]) -> bool {
        a != c && self.spatial.can_connect(p, a, b, c)
    }

    pub fn can_connect_temporally<V: Variant>(
        &self,
        p: &Problem<V>,
        i: Transport,
        j: Transport,
    ) -> bool {
        let [i, j] = [i, j].map(|t| p.transport_by_idx(t));
        let [_, b] = i.ori_des();
        let [b2, _] = j.ori_des();
        debug_assert_eq!(b, b2);

        let at = i.destination().time();
        let dt = j.origin().time();

        self.temporal.can_connect(p, b, at, dt)
    }
}
