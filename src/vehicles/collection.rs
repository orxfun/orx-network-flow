use crate::Variant;
use crate::indices::IdxMap;
use crate::vehicles::Vehicle;

pub struct Vehicles<V: Variant> {
    map: IdxMap<V::V, (), Vehicle>,
}

impl<V: Variant> Default for Vehicles<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<V: Variant> Vehicles<V> {
    pub fn push(&mut self, key: V::V) -> Vehicle {
        self.map.push_or_update(key, ())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn entries(&self) -> impl Iterator<Item = (Vehicle, &V::V)> {
        self.map.entries().map(|(space, key, _)| (space, key))
    }
}
