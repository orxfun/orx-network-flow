use crate::{Variant, space_time::SpaceTime};

pub struct VehicleData<V: Variant> {
    max_capacity: V::F,
}

impl<V: Variant> VehicleData<V> {
    pub fn new(max_capacity: V::F) -> Self {
        Self { max_capacity }
    }

    pub fn capacity(&self) -> V::F {
        self.max_capacity
    }
}
