use super::{CommodityLoad, CommodityPaths, SolutionBuilder};
use crate::{Problem, Variant, VecTransport, commodities::VecCommodity};
use alloc::vec::Vec;

#[derive(derive_new::new)]
pub struct McnfSolution<V: Variant> {
    commodity_paths: VecCommodity<CommodityPaths<V>>,
    transport_loads: VecTransport<Vec<CommodityLoad<V>>>,
}

impl<V: Variant> McnfSolution<V> {
    pub fn builder(p: &Problem<V>) -> SolutionBuilder<'_, V> {
        SolutionBuilder::new(p)
    }
}
