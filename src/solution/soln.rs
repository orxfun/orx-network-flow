use crate::solution::{CommodityFlows, SolutionBuilder};
use crate::{Variant, commodities::VecCommodity};

#[derive(derive_new::new)]
pub struct Solution<V: Variant> {
    commodity_flows: VecCommodity<CommodityFlows<V>>,
}

impl<V: Variant> Solution<V> {
    pub fn builder(len_commodities: usize) -> SolutionBuilder<V> {
        SolutionBuilder::new(len_commodities)
    }
}
