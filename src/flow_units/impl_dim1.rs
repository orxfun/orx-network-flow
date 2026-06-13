use crate::flow_units::FlowUnit;

impl FlowUnit for u64 {
    fn into_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        value as u64
    }

    fn inf() -> Self {
        Self::MAX
    }
}
