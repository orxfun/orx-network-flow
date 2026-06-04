use crate::{
    Problem, Variant, spaces::Space, std_utils::Map, time::Time, transports::Transport,
    vehicle_types::VehicleType,
};

struct CT {
    same_vehicle: Time,
    changed_vehicle: Time,
}

impl CT {
    pub fn new(same_vehicle: impl Into<Time>, changed_vehicle: impl Into<Time>) -> Self {
        Self {
            same_vehicle: same_vehicle.into(),
            changed_vehicle: changed_vehicle.into(),
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

pub struct ConnTimeBounds {
    global: CT,
    by_space: Map<Space, CT>,
    by_vehicle_type: Map<(VehicleType, VehicleType), CT>,
    by_space_vehicle_type: Map<(Space, VehicleType, VehicleType), CT>,
    by_transport: Map<(Transport, Transport), CT>,
}

impl ConnTimeBounds {
    pub fn new_min_conn_time() -> Self {
        Self {
            global: CT::new(Time::zero(), Time::zero()),
            by_space: Default::default(),
            by_vehicle_type: Default::default(),
            by_space_vehicle_type: Default::default(),
            by_transport: Default::default(),
        }
    }

    pub fn new_max_conn_time() -> Self {
        Self {
            global: CT::new(Time::inf(), Time::inf()),
            by_space: Default::default(),
            by_vehicle_type: Default::default(),
            by_space_vehicle_type: Default::default(),
            by_transport: Default::default(),
        }
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

pub enum ConnTimeBoundType {
    Min,
    Max,
}

pub struct ConnectionTimeBuilder<'a, V: Variant> {
    p: &'a mut Problem<V>,
    bound_type: ConnTimeBoundType,
}

impl<'a, V: Variant> ConnectionTimeBuilder<'a, V> {
    pub(crate) fn new(p: &'a mut Problem<V>, bound_type: ConnTimeBoundType) -> Self {
        Self { p, bound_type }
    }

    fn bounds(&mut self) -> &mut ConnTimeBounds {
        match self.bound_type {
            ConnTimeBoundType::Min => &mut self.p.time_bounds.min_conn_time,
            ConnTimeBoundType::Max => &mut self.p.time_bounds.max_conn_time,
        }
    }

    pub fn global(
        mut self,
        same_vehicle: impl Into<Time>,
        changed_vehicle: impl Into<Time>,
    ) -> Self {
        self.bounds().global = CT::new(same_vehicle, changed_vehicle);
        self
    }

    pub fn by_space(mut self, space: &V::S, same_vehicle: Time, changed_vehicle: Time) -> Self {
        let space = self
            .p
            .space_ind(space)
            .expect("Space '{space}' does not belong to the problem");
        let ct = CT::new(same_vehicle, changed_vehicle);
        self.bounds().by_space.insert(space, ct);
        self
    }

    pub fn by_vehicle(
        mut self,
        first_vehicle_type: &V::W,
        second_vehicle_type: &V::W,
        same_vehicle: Time,
        changed_vehicle: Time,
    ) -> Self {
        let v1 = self
            .p
            .vehicle_type_ind(first_vehicle_type)
            .expect("Vehicle type '{first_vehicle_type}' does not belong to the problem");
        let v2 = self
            .p
            .vehicle_type_ind(second_vehicle_type)
            .expect("Vehicle type '{second_vehicle_type}' does not belong to the problem");
        let ct = CT::new(same_vehicle, changed_vehicle);
        self.bounds().by_vehicle_type.insert((v1, v2), ct);
        self
    }

    pub fn by_space_vehicle(
        mut self,
        space: &V::S,
        first_vehicle_type: &V::W,
        second_vehicle_type: &V::W,
        same_vehicle: Time,
        changed_vehicle: Time,
    ) -> Self {
        let space = self
            .p
            .space_ind(space)
            .expect("Space '{space}' does not belong to the problem");
        let v1 = self
            .p
            .vehicle_type_ind(first_vehicle_type)
            .expect("Vehicle type '{first_vehicle_type}' does not belong to the problem");
        let v2 = self
            .p
            .vehicle_type_ind(second_vehicle_type)
            .expect("Vehicle type '{second_vehicle_type}' does not belong to the problem");
        let ct = CT::new(same_vehicle, changed_vehicle);
        self.bounds()
            .by_space_vehicle_type
            .insert((space, v1, v2), ct);
        self
    }

    pub fn by_transport(
        mut self,
        first_transport: &V::T,
        second_transport: &V::T,
        same_vehicle: Time,
        changed_vehicle: Time,
    ) -> Self {
        let t1 = self
            .p
            .transport_ind(first_transport)
            .expect("Transport '{first_transport}' does not belong to the problem");
        let t2 = self
            .p
            .transport_ind(second_transport)
            .expect("Transport '{second_transport}' does not belong to the problem");
        let ct = CT::new(same_vehicle, changed_vehicle);
        self.bounds().by_transport.insert((t1, t2), ct);
        self
    }
}
