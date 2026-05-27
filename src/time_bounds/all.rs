use crate::time_bounds::conn_time::ConnTime;

pub struct TimeBounds {
    pub min_conn_time: ConnTime,
    pub max_conn_time: ConnTime,
}
