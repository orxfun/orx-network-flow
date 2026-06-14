use crate::{Variant, solution_deprecated::SolutionDeprecated};

pub struct SolutionBuilder<V: Variant>(SolutionDeprecated<V>);

impl<V: Variant> SolutionBuilder<V> {
    pub fn new(len_commodities: usize) -> Self {
        let commodity_flows = (0..len_commodities).map(|_| Default::default()).collect();
        Self(SolutionDeprecated::new(commodity_flows))
    }

    pub fn finish(self) -> SolutionDeprecated<V> {
        self.0
    }
}
