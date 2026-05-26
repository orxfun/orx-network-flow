use crate::Variant;
use crate::indices::IdxMap;
use crate::vehicle_types::{VehicleType, VehicleTypeData};

pub struct VehicleTypes<V: Variant> {
    map: IdxMap<V::V, VehicleTypeData<V>, VehicleType>,
}

impl<V: Variant> Default for VehicleTypes<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<V: Variant> VehicleTypes<V> {
    pub fn push(&mut self, key: V::V, maximum_capacity: V::F) -> VehicleType {
        let data = VehicleTypeData::new(maximum_capacity);
        self.map.push_or_update(key, data)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_by_key(&self, key: &V::V) -> Option<&VehicleTypeData<V>> {
        self.map.get_by_key(key)
    }

    pub fn entries(&self) -> impl Iterator<Item = (VehicleType, &V::V, &VehicleTypeData<V>)> {
        self.map.entries()
    }
}
