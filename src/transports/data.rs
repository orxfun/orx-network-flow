use crate::{Variant, space_time::SpaceTime, vehicles::Vehicle};

pub struct TransportData<V: Variant> {
    vehicle: Vehicle,
    ori: SpaceTime,
    des: SpaceTime,
    cap: V::F,
}

impl<V: Variant> TransportData<V> {
    pub fn new(vehicle: Vehicle, ori: SpaceTime, des: SpaceTime, cap: V::F) -> Self {
        Self {
            vehicle,
            ori,
            des,
            cap,
        }
    }

    pub fn origin(&self) -> SpaceTime {
        self.ori
    }

    pub fn destination(&self) -> SpaceTime {
        self.des
    }

    pub fn vehicle(&self) -> Vehicle {
        self.vehicle
    }

    pub fn capacity(&self) -> V::F {
        self.cap
    }
}
