use crate::problem::min_connection_time::MinConnectionTime;
use crate::problem::space_connectivity::SpaceConnectivity;
use crate::spaces::Space;
use crate::transports::Transport;
use crate::{Problem, Variant};

#[derive(Default)]
pub struct Connectivity<V: Variant> {
    pub space: SpaceConnectivity<V>,
    pub min_conn_time: MinConnectionTime,
}

impl<V: Variant> Connectivity<V> {
    pub fn can_connect_by_space(&self, p: &Problem<V>, [a, b, c]: [Space; 3]) -> bool {
        a != c && self.space.can_connect(p, a, b, c)
    }

    pub fn can_connect_by_time(&self, p: &Problem<V>, i: Transport, j: Transport) -> bool {
        let [i, j] = [i, j].map(|t| p.transport_by_idx(t));
        let [_, b] = i.ori_des();
        let [b2, _] = j.ori_des();
        debug_assert_eq!(b, b2);

        let at = i.destination().time();
        let dt = j.origin().time();

        self.min_conn_time.can_connect(b, at, dt)
    }
}
