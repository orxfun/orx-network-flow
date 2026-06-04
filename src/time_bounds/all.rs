use crate::time::Time;
use crate::time_bounds::waiting_at_source::WaitingAtSourceBounds;
use crate::time_bounds::{conn_time::ConnTimeBounds, lateness_earliness::LatenessEarlinessBounds};

pub struct TimeBounds {
    pub min_conn_time: ConnTimeBounds,
    pub max_conn_time: ConnTimeBounds,
    pub max_lateness: LatenessEarlinessBounds,
    pub max_earliness: LatenessEarlinessBounds,
    pub max_waiting: WaitingAtSourceBounds,
}

impl Default for TimeBounds {
    fn default() -> Self {
        Self {
            min_conn_time: Default::default(),
            max_conn_time: Default::default(),
            max_lateness: LatenessEarlinessBounds::new(Time::inf()),
            max_earliness: LatenessEarlinessBounds::new(Time::inf()),
            max_waiting: WaitingAtSourceBounds::new(Time::inf()),
        }
    }
}
