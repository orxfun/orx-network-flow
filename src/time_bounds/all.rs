use crate::time_bounds::arrival::ArrivalBounds;
use crate::time_bounds::departure::DepartureBounds;

pub struct TimeBounds {
    pub max_lateness: ArrivalBounds,
    pub max_earliness: ArrivalBounds,
    pub max_waiting: DepartureBounds,
}

impl Default for TimeBounds {
    fn default() -> Self {
        Self {
            max_lateness: ArrivalBounds::new_lateness(),
            max_earliness: ArrivalBounds::new_earliness(),
            max_waiting: DepartureBounds::new(),
        }
    }
}
