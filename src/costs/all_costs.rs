use crate::Variant;
use crate::cost::Cost;
use crate::costs::{earliness_cost::EarlinessCost, lateness_cost::LatenessCost};
use crate::costs::{lost_revenue::LostRevenue, transport_cost::TransportCost};

pub struct Costs<V: Variant> {
    pub earliness: EarlinessCost<V>,
    pub lateness: LatenessCost<V>,
    pub lost_revenue: LostRevenue<V>,
    pub transport: TransportCost<V>,
}

impl<V: Variant> Default for Costs<V> {
    fn default() -> Self {
        Self::new(Cost::zero(), Cost::zero(), Cost::zero(), Cost::zero())
    }
}

impl<V: Variant> Costs<V> {
    pub fn new(
        global_earliness_cost_per_unit: V::C,
        global_lateness_cost_per_unit: V::C,
        global_revenue_per_unit: V::C,
        global_transport_unit_cost: V::C,
    ) -> Self {
        Self {
            earliness: EarlinessCost::new(global_earliness_cost_per_unit),
            lateness: LatenessCost::new(global_lateness_cost_per_unit),
            lost_revenue: LostRevenue::new(global_revenue_per_unit),
            transport: TransportCost::new(global_transport_unit_cost),
        }
    }
}
