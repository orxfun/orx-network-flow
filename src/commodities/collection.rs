use crate::Variant;
use crate::commodities::{Commodity, CommodityData};
use crate::indices::IdxMap;
use crate::space_time::SpaceTime;

pub struct Commodities<V: Variant> {
    map: IdxMap<V::K, CommodityData<V>, Commodity>,
}

impl<V: Variant> Default for Commodities<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<V: Variant> Commodities<V> {
    pub fn push(&mut self, key: V::K, ori: SpaceTime, des: SpaceTime, amount: V::F) -> Commodity {
        let data = CommodityData::new(ori, des, amount);
        self.map.push_or_update(key, data)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_by_key(&self, key: V::K) -> Option<&CommodityData<V>> {
        self.map.get_by_key(key)
    }

    pub fn entries(&self) -> impl Iterator<Item = (Commodity, &V::K, &CommodityData<V>)> {
        self.map.entries()
    }
}
