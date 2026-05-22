use crate::spaces::{Space, SpaceData};
use crate::{indices::IndexMap, std_utils::MapKey};

pub struct Spaces<K: MapKey, D: SpaceData> {
    map: IndexMap<K, D, Space>,
}

impl<K: MapKey, D: SpaceData> Spaces<K, D> {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    pub fn push_or_update(&mut self, key: K, data: D) -> Space {
        self.map.push_or_update(key, data)
    }
}
