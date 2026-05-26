use crate::{
    Problem, Variant, spaces::Space, std_utils::Map, time::Time, transports::Transport,
    vehicle_types::VehicleType,
};

struct MinConnTime {
    same_vehicle: Time,
    changed_vehicle: Time,
}

impl MinConnTime {
    pub fn new(same_vehicle: Time, changed_vehicle: Time) -> Self {
        Self {
            same_vehicle,
            changed_vehicle,
        }
    }
}

impl Default for MinConnTime {
    fn default() -> Self {
        Self::new(Time::from(0u64), Time::from(0u64))
    }
}

impl MinConnTime {
    fn conn_time(&self, same_vehicle: bool) -> Time {
        match same_vehicle {
            true => self.same_vehicle,
            false => self.changed_vehicle,
        }
    }
}

#[derive(Default)]
pub struct ConnectionTime {
    global: MinConnTime,
    by_space: Map<Space, MinConnTime>,
    by_vehicle_type: Map<(VehicleType, VehicleType), MinConnTime>,
    by_space_vehicle_type: Map<(Space, VehicleType, VehicleType), MinConnTime>,
    by_transport: Map<(Transport, Transport), MinConnTime>,
}

impl ConnectionTime {
    pub fn new(same_vehicle: Time, changed_vehicle: Time) -> Self {
        let global = MinConnTime {
            same_vehicle,
            changed_vehicle,
        };
        Self {
            global,
            ..Default::default()
        }
    }

    pub fn space_specific(&mut self, space: Space, same_vehicle: Time, changed_vehicle: Time) {
        let ct = MinConnTime::new(same_vehicle, changed_vehicle);
        self.by_space.insert(space, ct);
    }

    pub fn vehicle_type_specific(
        &mut self,
        first_vehicle_type: VehicleType,
        second_vehicle_type: VehicleType,
        same_vehicle: Time,
        changed_vehicle: Time,
    ) {
        let ct = MinConnTime::new(same_vehicle, changed_vehicle);
        self.by_vehicle_type
            .insert((first_vehicle_type, second_vehicle_type), ct);
    }

    pub fn space_and_vehicle_type_specific(
        &mut self,
        space: Space,
        first_vehicle_type: VehicleType,
        second_vehicle_type: VehicleType,
        same_vehicle: Time,
        changed_vehicle: Time,
    ) {
        let ct = MinConnTime::new(same_vehicle, changed_vehicle);
        self.by_space_vehicle_type
            .insert((space, first_vehicle_type, second_vehicle_type), ct);
    }

    pub fn transport_specific(
        &mut self,
        first_transport: Transport,
        second_transport: Transport,
        same_vehicle: Time,
        changed_vehicle: Time,
    ) {
        let ct = MinConnTime::new(same_vehicle, changed_vehicle);
        self.by_transport
            .insert((first_transport, second_transport), ct);
    }

    pub fn conn_time<V: Variant>(&self, prob: &Problem<V>, f: Transport, g: Transport) -> Time {
        debug_assert_eq!(
            prob.transport_by_idx(f).destination().space(),
            prob.transport_by_idx(g).origin().space()
        );

        let vehicle_f = prob.transport_by_idx(f).vehicle();
        let vehicle_g = prob.transport_by_idx(g).vehicle();
        let same_vehicle = vehicle_f == vehicle_g;

        if let Some(ct) = self.by_transport.get(&(f, g)) {
            return ct.conn_time(same_vehicle);
        }

        let space = prob.transport_by_idx(f).destination().space();
        let vehicle_type_f = prob.vehicle_by_idx(vehicle_f).vehicle_type();
        let vehicle_type_g = prob.vehicle_by_idx(vehicle_g).vehicle_type();
        if let Some(ct) = self
            .by_space_vehicle_type
            .get(&(space, vehicle_type_f, vehicle_type_g))
        {
            return ct.conn_time(same_vehicle);
        }

        if let Some(ct) = self.by_vehicle_type.get(&(vehicle_type_f, vehicle_type_g)) {
            return ct.conn_time(same_vehicle);
        }

        if let Some(ct) = self.by_space.get(&space) {
            return ct.conn_time(same_vehicle);
        }

        self.global.conn_time(same_vehicle)
    }
}
