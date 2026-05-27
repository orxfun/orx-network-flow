use crate::time_bounds::{conn_time::ConnTimeBounds, lateness_earliness::LatenessEarlinessBounds};

pub struct TimeBounds {
    pub min_conn_time: ConnTimeBounds,
    pub max_conn_time: ConnTimeBounds,
    pub min_lateness: LatenessEarlinessBounds,
    pub max_lateness: LatenessEarlinessBounds,
    pub min_earliness: LatenessEarlinessBounds,
    pub max_earliness: LatenessEarlinessBounds,
    pub max_waiting: LatenessEarlinessBounds,
}
