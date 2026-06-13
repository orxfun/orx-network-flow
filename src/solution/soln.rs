use crate::{Variant, commodities::VecCommodity, solution::CommodityFlows};

pub struct Solution<V: Variant> {
    commodity_flows: VecCommodity<CommodityFlows<V>>,
}
