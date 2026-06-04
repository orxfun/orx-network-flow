use crate::{
    Problem, Variant, spaces::Space, std_utils::Map, time::Time, transports::Transport,
    vehicle_types::VehicleType,
};

struct CT {
    same_vehicle: Time,
    changed_vehicle: Time,
}

impl CT {
    pub fn new(same_vehicle: Time, changed_vehicle: Time) -> Self {
        Self {
            same_vehicle,
            changed_vehicle,
        }
    }
}

impl Default for CT {
    fn default() -> Self {
        Self::new(Time::from(0i64), Time::from(0i64))
    }
}

impl CT {
    fn conn_time(&self, same_vehicle: bool) -> Time {
        match same_vehicle {
            true => self.same_vehicle,
            false => self.changed_vehicle,
        }
    }
}

#[derive(Default)]
pub struct ConnTimeBounds {
    global: CT,
    by_space: Map<Space, CT>,
    by_vehicle_type: Map<(VehicleType, VehicleType), CT>,
    by_space_vehicle_type: Map<(Space, VehicleType, VehicleType), CT>,
    by_transport: Map<(Transport, Transport), CT>,
}

impl ConnTimeBounds {
    pub fn new(global_same_vehicle: Time, global_changed_vehicle: Time) -> Self {
        Self {
            global: CT::new(global_same_vehicle, global_changed_vehicle),
            ..Default::default()
        }
    }

    pub fn space_specific(&mut self, space: Space, same_vehicle: Time, changed_vehicle: Time) {
        let ct = CT::new(same_vehicle, changed_vehicle);
        self.by_space.insert(space, ct);
    }

    pub fn vehicle_type_specific(
        &mut self,
        first_vehicle_type: VehicleType,
        second_vehicle_type: VehicleType,
        same_vehicle: Time,
        changed_vehicle: Time,
    ) {
        let ct = CT::new(same_vehicle, changed_vehicle);
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
        let ct = CT::new(same_vehicle, changed_vehicle);
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
        let ct = CT::new(same_vehicle, changed_vehicle);
        self.by_transport
            .insert((first_transport, second_transport), ct);
    }

    pub fn bound<V: Variant>(&self, prob: &Problem<V>, f: Transport, g: Transport) -> Time {
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

pub struct ConnectionTimeBuilder<'a, V: Variant>(&'a mut Problem<V>);

impl<'a, V: Variant> ConnectionTimeBuilder<'a, V> {
    pub(crate) fn new(prob: &'a mut Problem<V>) -> Self {
        Self(prob)
    }

    fn bounds(&mut self) -> &mut ConnTimeBounds {
        &mut self.0.time_bounds.min_conn_time
    }
}
