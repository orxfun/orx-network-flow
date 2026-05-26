use crate::cost::Cost;
use crate::vehicle_types::VehicleType;
use crate::{Problem, Variant};
use crate::{commodities::Commodity, std_utils::Map, transports::Transport, vehicles::Vehicle};

pub struct TransportationCost<V: Variant> {
    global: V::C,
    by_commodity: Map<Commodity, V::C>,
    by_vehicle: Map<Vehicle, V::C>,
    by_transport: Map<Transport, V::C>,
    by_commodity_vehicle_type: Map<(Commodity, VehicleType), V::C>,
    by_commodity_transport: Map<(Commodity, Transport), V::C>,
}

impl<V: Variant> Default for TransportationCost<V> {
    fn default() -> Self {
        Self::new(Cost::zero())
    }
}

impl<V: Variant> TransportationCost<V> {
    pub fn new(default_unit_cost: V::C) -> Self {
        Self {
            global: default_unit_cost,
            by_commodity: Default::default(),
            by_vehicle: Default::default(),
            by_transport: Default::default(),
            by_commodity_vehicle_type: Default::default(),
            by_commodity_transport: Default::default(),
        }
    }

    pub fn commodity_specific(&mut self, commodity: Commodity, unit_cost: V::C) {
        self.by_commodity.insert(commodity, unit_cost);
    }

    pub fn vehicle_specific(&mut self, vehicle: Vehicle, unit_cost: V::C) {
        self.by_vehicle.insert(vehicle, unit_cost);
    }

    pub fn transport_specific(&mut self, transport: Transport, unit_cost: V::C) {
        self.by_transport.insert(transport, unit_cost);
    }

    pub fn commodity_vehicle_specific(
        &mut self,
        commodity: Commodity,
        vehicle_type: VehicleType,
        unit_cost: V::C,
    ) {
        self.by_commodity_vehicle_type
            .insert((commodity, vehicle_type), unit_cost);
    }

    pub fn commodity_transport_specific(
        &mut self,
        commodity: Commodity,
        transport: Transport,
        unit_cost: V::C,
    ) {
        self.by_commodity_transport
            .insert((commodity, transport), unit_cost);
    }

    pub fn cost(&self, prob: &Problem<V>, commodity: Commodity, transport: Transport) -> V::C {
        if let Some(cost) = self.by_commodity_transport.get(&(commodity, transport)) {
            return *cost;
        }

        let vehicle = prob.transport_by_idx(transport).vehicle();
        let vehicle_type = prob.vehicle_by_idx(vehicle).vehicle_type();
        if let Some(cost) = self
            .by_commodity_vehicle_type
            .get(&(commodity, vehicle_type))
        {
            return *cost;
        }

        if let Some(cost) = self.by_transport.get(&transport) {
            return *cost;
        }

        if let Some(cost) = self.by_vehicle.get(&vehicle) {
            return *cost;
        }

        if let Some(cost) = self.by_commodity.get(&commodity) {
            return *cost;
        }

        self.global
    }
}
