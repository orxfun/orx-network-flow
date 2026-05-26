use crate::Variant;
use crate::{commodities::Commodity, spaces::Space, std_utils::Map, transports::Transport};
use num::Zero;

pub struct LatenessCost<V: Variant> {
    global_per_unit: V::C,
    by_commodity_per_unit: Map<Commodity, V::C>,
    by_destination_per_unit: Map<Space, V::C>,
    by_commodity_destination_per_unit: Map<(Commodity, Space), V::C>,
    by_commodity_transport: Map<(Commodity, Transport), V::C>,
}

impl<V: Variant> Default for LatenessCost<V> {
    fn default() -> Self {
        Self::new(Zero::zero())
    }
}

impl<V: Variant> LatenessCost<V> {
    pub fn new(global_lateness_cost_per_unit: V::C) -> Self {
        Self {
            global_per_unit: global_lateness_cost_per_unit,
            by_commodity_per_unit: Default::default(),
            by_destination_per_unit: Default::default(),
            by_commodity_destination_per_unit: Default::default(),
            by_commodity_transport: Default::default(),
        }
    }
}
