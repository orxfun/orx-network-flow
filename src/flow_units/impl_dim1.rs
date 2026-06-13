use crate::flow_units::FlowUnit;

impl FlowUnit for u64 {
    fn into_f64(self) -> f64 {
        self as f64
    }
}
