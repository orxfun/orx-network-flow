use crate::{Problem, Variant};
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

    pub fn commodity_specific(&mut self, commodity: Commodity, lateness_cost_per_unit: V::C) {
        self.by_commodity_per_unit
            .insert(commodity, lateness_cost_per_unit);
    }

    pub fn destination_specific(&mut self, destination: Space, lateness_cost_per_unit: V::C) {
        self.by_destination_per_unit
            .insert(destination, lateness_cost_per_unit);
    }

    pub fn commodity_destination_specific(
        &mut self,
        commodity: Commodity,
        destination: Space,
        lateness_cost_per_unit: V::C,
    ) {
        self.by_commodity_destination_per_unit
            .insert((commodity, destination), lateness_cost_per_unit);
    }

    pub fn commodity_transport_specific(
        &mut self,
        commodity: Commodity,
        transport: Transport,
        lateness_cost: V::C,
    ) {
        self.by_commodity_transport
            .insert((commodity, transport), lateness_cost);
    }

    pub fn cost(&self, prob: &Problem<V>, commodity: Commodity, transport: Transport) -> V::C {
        debug_assert_eq!(
            prob.commodity_by_idx(commodity).destination().space(),
            prob.transport_by_idx(transport).destination().space()
        );

        let commodity_des = prob.commodity_by_idx(commodity).destination();
        let transport_des = prob.transport_by_idx(transport).destination();

        if transport_des.time() <= commodity_des.time() {
            return Zero::zero();
        }

        if let Some(cost) = self.by_commodity_transport.get(&(commodity, transport)) {
            return *cost;
        }

        let des = transport_des.space();
        if let Some(cost) = self
            .by_commodity_destination_per_unit
            .get(&(commodity, des))
        {
            return *cost;
        }

        if let Some(cost) = self.by_destination_per_unit.get(&des) {
            return *cost;
        }

        if let Some(cost) = self.by_commodity_per_unit.get(&commodity) {
            return *cost;
        }

        self.global_per_unit
    }
}
