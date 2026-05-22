use crate::transports::{Transport, TransportData};
use crate::{indices::IndexMap, std_utils::MapKey};

pub struct Transports<K: MapKey> {
    map: IndexMap<K, TransportData, Transport>,
}

impl<K: MapKey> Default for Transports<K> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<K: MapKey> Transports<K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, key: K, data: TransportData) -> Transport {
        self.map.push_or_update(key, data)
    }
}
