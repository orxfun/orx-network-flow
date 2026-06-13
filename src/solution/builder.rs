use crate::{Variant, solution::Solution};

pub struct SolutionBuilder<V: Variant>(Solution<V>);

impl<V: Variant> SolutionBuilder<V> {
    pub fn new(len_commodities: usize) -> Self {
        let commodity_flows = (0..len_commodities).map(|_| Default::default()).collect();
        Self(Solution::new(commodity_flows))
    }

    pub fn finish(self) -> Solution<V> {
        self.0
    }
}
