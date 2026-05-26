use crate::cost::Cost;
use crate::time::Time;
use crate::{Problem, Variant};
use crate::{commodities::Commodity, spaces::Space, std_utils::Map, transports::Transport};

pub struct EarlinessCost<V: Variant> {
    global_per_unit: V::C,
    by_commodity_per_unit: Map<Commodity, V::C>,
    by_destination_per_unit: Map<Space, V::C>,
    by_commodity_destination_per_unit: Map<(Commodity, Space), V::C>,
    by_commodity_transport: Map<(Commodity, Transport), V::C>,
}

impl<V: Variant> Default for EarlinessCost<V> {
    fn default() -> Self {
        Self::new(Cost::zero())
    }
}

impl<V: Variant> EarlinessCost<V> {
    pub fn new(global_earliness_cost_per_unit: V::C) -> Self {
        Self {
            global_per_unit: global_earliness_cost_per_unit,
            by_commodity_per_unit: Default::default(),
            by_destination_per_unit: Default::default(),
            by_commodity_destination_per_unit: Default::default(),
            by_commodity_transport: Default::default(),
        }
    }

    pub fn commodity_specific(&mut self, commodity: Commodity, earliness_cost_per_unit: V::C) {
        self.by_commodity_per_unit
            .insert(commodity, earliness_cost_per_unit);
    }

    pub fn destination_specific(&mut self, destination: Space, earliness_cost_per_unit: V::C) {
        self.by_destination_per_unit
            .insert(destination, earliness_cost_per_unit);
    }

    pub fn commodity_destination_specific(
        &mut self,
        commodity: Commodity,
        destination: Space,
        earliness_cost_per_unit: V::C,
    ) {
        self.by_commodity_destination_per_unit
            .insert((commodity, destination), earliness_cost_per_unit);
    }

    pub fn commodity_transport_specific(
        &mut self,
        commodity: Commodity,
        transport: Transport,
        earliness_cost: V::C,
    ) {
        self.by_commodity_transport
            .insert((commodity, transport), earliness_cost);
    }

    pub fn cost(&self, prob: &Problem<V>, commodity: Commodity, transport: Transport) -> V::C {
        debug_assert_eq!(
            prob.commodity_by_idx(commodity).destination().space(),
            prob.transport_by_idx(transport).destination().space()
        );

        let commodity_des = prob.commodity_by_idx(commodity).destination();
        let transport_des = prob.transport_by_idx(transport).destination();

        let earliness = commodity_des.time() - transport_des.time();

        if earliness <= Time::zero() {
            return Cost::zero();
        }

        if let Some(cost) = self.by_commodity_transport.get(&(commodity, transport)) {
            return *cost;
        }

        let des = transport_des.space();
        if let Some(cost) = self
            .by_commodity_destination_per_unit
            .get(&(commodity, des))
        {
            return *cost * earliness;
        }

        if let Some(cost) = self.by_destination_per_unit.get(&des) {
            return *cost * earliness;
        }

        if let Some(cost) = self.by_commodity_per_unit.get(&commodity) {
            return *cost * earliness;
        }

        self.global_per_unit * earliness
    }
}
