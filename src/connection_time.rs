use crate::{
    Problem, Variant, spaces::Space, std_utils::Map, time::Time, transports::Transport,
    vehicle_types::VehicleType,
};

struct MinConnTime {
    same_vehicle: Time,
    changed_vehicle: Time,
}

impl MinConnTime {
    fn conn_time(&self, same_vehicle: bool) -> Time {
        match same_vehicle {
            true => self.same_vehicle,
            false => self.changed_vehicle,
        }
    }
}

pub struct ConnectionTime {
    global: MinConnTime,
    by_space: Map<Space, MinConnTime>,
    by_vehicle_type: Map<(VehicleType, VehicleType), MinConnTime>,
    by_space_vehicle_type: Map<(Space, VehicleType, VehicleType), MinConnTime>,
    by_transport: Map<(Transport, Transport), MinConnTime>,
}

impl ConnectionTime {
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
