use crate::solution_deprecated::{CommodityFlows, SolutionBuilder};
use crate::{Variant, commodities::VecCommodity};

#[derive(derive_new::new)]
pub struct SolutionDeprecated<V: Variant> {
    commodity_flows: VecCommodity<CommodityFlows<V>>,
}

impl<V: Variant> SolutionDeprecated<V> {
    pub fn builder(len_commodities: usize) -> SolutionBuilder<V> {
        SolutionBuilder::new(len_commodities)
    }
}
