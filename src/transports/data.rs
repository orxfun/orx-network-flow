use crate::{Variant, space_time::SpaceTime, vehicle_types::VehicleType};

pub struct TransportData<V: Variant> {
    vehicle_type: VehicleType,
    ori: SpaceTime,
    des: SpaceTime,
    cap: V::F,
}

impl<V: Variant> TransportData<V> {
    pub fn new(vehicle_type: VehicleType, ori: SpaceTime, des: SpaceTime, cap: V::F) -> Self {
        Self {
            vehicle_type,
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

    pub fn vehicle_type(&self) -> VehicleType {
        self.vehicle_type
    }

    pub fn capacity(&self) -> V::F {
        self.cap
    }
}
