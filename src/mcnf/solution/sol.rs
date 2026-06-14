use super::{CommodityPaths, SolutionBuilder};
use crate::{Commodity, Problem, Variant, VecTransport, commodities::VecCommodity};
use alloc::vec::Vec;

pub struct CommodityLoad<V: Variant> {
    pub commodity: Commodity,
    pub load: V::F,
}

#[derive(derive_new::new)]
pub struct Solution<V: Variant> {
    commodity_paths: VecCommodity<CommodityPaths<V>>,
    transport_loads: VecTransport<Vec<CommodityLoad<V>>>,
}

impl<V: Variant> Solution<V> {
    pub fn builder(p: &Problem<V>) -> SolutionBuilder<'_, V> {
        SolutionBuilder::new(p)
    }
}
