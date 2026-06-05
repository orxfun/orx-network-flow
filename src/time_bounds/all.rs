use crate::time_bounds::departure::DepartureBounds;
use crate::time_bounds::{arrival::ArrivalBounds, conn_time::ConnTimeBounds};

pub struct TimeBounds {
    pub min_conn_time: ConnTimeBounds,
    pub max_conn_time: ConnTimeBounds,
    pub max_lateness: ArrivalBounds,
    pub max_earliness: ArrivalBounds,
    pub max_waiting: DepartureBounds,
}

impl Default for TimeBounds {
    fn default() -> Self {
        Self {
            min_conn_time: ConnTimeBounds::new_min_conn_time(),
            max_conn_time: ConnTimeBounds::new_max_conn_time(),
            max_lateness: ArrivalBounds::new_lateness(),
            max_earliness: ArrivalBounds::new_earliness(),
            max_waiting: DepartureBounds::new(),
        }
    }
}
