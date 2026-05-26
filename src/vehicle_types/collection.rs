use crate::Variant;
use crate::indices::IdxMap;
use crate::vehicle_types::VehicleType;

pub struct VehicleTypes<V: Variant> {
    map: IdxMap<V::V, (), VehicleType>,
}

impl<V: Variant> Default for VehicleTypes<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<V: Variant> VehicleTypes<V> {
    pub fn push(&mut self, key: V::V) -> VehicleType {
        self.map.push_or_update(key, ())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn entries(&self) -> impl Iterator<Item = (VehicleType, &V::V)> {
        self.map.entries().map(|(space, key, _)| (space, key))
    }
}
