use crate::Variant;
use crate::indices::IdxMap;
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, key: V::T, data: TransportData<V>) -> Transport {
        self.map.push_or_update(key, data)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_by_key(&self, key: V::T) -> Option<&TransportData<V>> {
        self.map.get_by_key(key)
    }
}
