use super::Solution;
use crate::Variant;

pub struct SolutionBuilder<V: Variant>(Solution<V>);

impl<V: Variant> SolutionBuilder<V> {
    pub fn new(len_commodities: usize) -> Self {
        let commodity_flows = (0..len_commodities).map(|_| Default::default()).collect();
        let transport_loads = (0..len_commodities).map(|_| Default::default()).collect();
        Self(Solution::new(commodity_flows, transport_loads))
    }

    pub fn finish(self) -> Solution<V> {
        self.0
    }

    // mut
}
