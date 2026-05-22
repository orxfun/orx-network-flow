use crate::commodities::{Commodity, CommodityData};
use crate::indices::IndexMap;
use crate::space_time::SpaceTime;
use crate::std_utils::MapKey;

pub struct Commodities<K: MapKey> {
    map: IndexMap<K, CommodityData, Commodity>,
}

impl<K: MapKey> Default for Commodities<K> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<K: MapKey> Commodities<K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, key: K, ori: SpaceTime, des: SpaceTime) -> Commodity {
        let data = CommodityData::new(ori, des);
        self.map.push_or_update(key, data)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_by_key(&self, key: K) -> Option<&CommodityData> {
        self.map.get_by_key(key)
    }

    pub fn entries(&self) -> impl Iterator<Item = (Commodity, &K, &CommodityData)> {
        self.map.entries()
    }
}
