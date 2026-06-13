use crate::{Variant, commodities::VecCommodity, solution::CommodityFlows};

#[derive(derive_new::new)]
pub struct Solution<V: Variant> {
    commodity_flows: VecCommodity<CommodityFlows<V>>,
}
