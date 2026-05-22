use crate::spaces::{Space, SpaceData};
use crate::{indices::IndexMap, std_utils::MapKey};

pub struct Spaces<K: MapKey, D: SpaceData> {
    map: IndexMap<K, D, Space>,
}

impl<K: MapKey, D: SpaceData> Default for Spaces<K, D> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<K: MapKey, D: SpaceData> Spaces<K, D> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_or_update(&mut self, key: K, data: D) -> Space {
        self.map.push_or_update(key, data)
    }
}
