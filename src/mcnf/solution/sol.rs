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

    pub fn commodity_paths(&self) -> &VecCommodity<CommodityPaths<V>> {
        &self.commodity_paths
    }

    pub fn transport_loads(&self) -> &VecTransport<Vec<CommodityLoad<V>>> {
        &self.transport_loads
    }
}
