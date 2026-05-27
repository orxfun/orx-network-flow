use crate::Variant;
use crate::costs::{earliness_cost::EarlinessCost, lateness_cost::LatenessCost};
use crate::costs::{lost_revenue::LostRevenue, transport_cost::TransportCost};

pub struct Cost<V: Variant> {
    earliness: EarlinessCost<V>,
    lateness: LatenessCost<V>,
    lost_revenue: LostRevenue<V>,
    transport: TransportCost<V>,
}

impl<V: Variant> Default for Cost<V> {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}

impl<V: Variant> Cost<V> {
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

    pub fn earliness_mut(&mut self) -> &mut EarlinessCost<V> {
        &mut self.earliness
    }

    pub fn lateness_mut(&mut self) -> &mut LatenessCost<V> {
        &mut self.lateness
    }

    pub fn lost_revenue_mut(&mut self) -> &mut LostRevenue<V> {
        &mut self.lost_revenue
    }

    pub fn transport_mut(&mut self) -> &mut TransportCost<V> {
        &mut self.transport
    }
}
