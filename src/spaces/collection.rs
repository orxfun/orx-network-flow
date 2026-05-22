use crate::spaces::Space;
use crate::{indices::IndexMap, std_utils::MapKey};

pub struct Spaces<K: MapKey> {
    map: IndexMap<K, (), Space>,
}

impl<K: MapKey> Default for Spaces<K> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<K: MapKey> Spaces<K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, key: K) -> Space {
        self.map.push_or_update(key, ())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
