use crate::Variant;

pub struct VehicleTypeData<V: Variant> {
    max_capacity: V::F,
}

impl<V: Variant> VehicleTypeData<V> {
    pub fn new(max_capacity: V::F) -> Self {
        Self { max_capacity }
    }

    pub fn capacity(&self) -> V::F {
        self.max_capacity
    }
}
