use crate::Variant;
use crate::indices::IdxMap;
use crate::vehicle_types::VehicleType;
use crate::vehicles::{Vehicle, VehicleData};

pub struct Vehicles<V: Variant> {
    map: IdxMap<V::V, VehicleData, Vehicle>,
}

impl<V: Variant> Default for Vehicles<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<V: Variant> Vehicles<V> {
    pub fn push(&mut self, key: V::V, vehicle_type: VehicleType) -> Vehicle {
        self.map.push_or_update(key, VehicleData::new(vehicle_type))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn entries(&self) -> impl Iterator<Item = (Vehicle, &V::V)> {
        self.map.entries().map(|(space, key, _)| (space, key))
    }

    pub fn get_by_key(&self, key: &V::V) -> Option<&VehicleData> {
        self.map.get_val_by_key(key)
    }

    pub fn get_ind_by_key(&self, key: &V::V) -> Option<Vehicle> {
        self.map.key_to_idx(key)
    }

    pub fn get_by_idx(&self, idx: Vehicle) -> Option<&VehicleData> {
        self.map.get_by_idx(idx)
    }
}
