use crate::{indices::IndexMap, space::Space, std_utils::MapKey};

pub struct Spaces<K: MapKey, V> {
    map: IndexMap<K, V, Space>,
}

impl<K: MapKey, V> Spaces<K, V> {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    pub fn push_or_update(&mut self, key: K, data: V) -> Space {
        self.map.push_or_update(key, data)
    }
}
