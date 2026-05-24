use crate::Variant;
use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use crate::transports::{Transport, TransportData};

pub struct Transports<V: Variant> {
    map: IdxMap<V::T, TransportData<V>, Transport>,
}

impl<V: Variant> Default for Transports<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<V: Variant> Transports<V> {
    pub fn push(&mut self, key: V::T, ori: SpaceTime, des: SpaceTime, capacity: V::F) -> Transport {
        let data = TransportData::new(ori, des, capacity);
        self.map.push_or_update(key, data)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_by_key(&self, key: &V::T) -> Option<&TransportData<V>> {
        self.map.get_by_key(key)
    }
}
