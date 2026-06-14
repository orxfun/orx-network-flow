use super::{CommodityPaths, SolutionBuilder};
use crate::{Commodity, Variant, VecTransport, commodities::VecCommodity};
use alloc::vec::Vec;

#[derive(derive_new::new)]
pub struct Solution<V: Variant> {
    commodity_paths: VecCommodity<CommodityPaths<V>>,
    transport_loads: VecTransport<Vec<CommodityLoad<V>>>,
}

impl<V: Variant> Solution<V> {
    pub fn builder(len_commodities: usize) -> SolutionBuilder<V> {
        SolutionBuilder::new(len_commodities)
    }
}

pub struct CommodityLoad<V: Variant> {
    pub commodity: Commodity,
    pub load: V::F,
}
