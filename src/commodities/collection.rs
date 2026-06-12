use crate::Variant;
use crate::commodities::{Commodity, CommodityData};
use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use core::ops::Deref;

pub struct Commodities<V: Variant> {
    pub(super) idx_map: IdxMap<V::K, CommodityData<V>, Commodity>,
}

impl<V: Variant> Default for Commodities<V> {
    fn default() -> Self {
        Self {
            idx_map: Default::default(),
        }
    }
}

impl<V: Variant> Commodities<V> {
    pub fn push(&mut self, key: V::K, ori: SpaceTime, des: SpaceTime, amount: V::F) -> Commodity {
        let data = CommodityData::new(ori, des, amount);
        self.idx_map.push_or_update(key, data)
    }

    pub fn get_by_key(&self, key: &V::K) -> Option<&CommodityData<V>> {
        self.idx_map.value_by_key(key)
    }

    pub fn get_ind_by_key(&self, key: &V::K) -> Option<Commodity> {
        self.idx_map.key_to_idx(key)
    }

    pub fn key(&self, idx: Commodity) -> Option<&V::K> {
        self.idx_map.key(idx)
    }

    pub fn get_by_idx(&self, idx: Commodity) -> Option<&CommodityData<V>> {
        self.idx_map.value(idx)
    }
}

impl<V: Variant> Deref for Commodities<V> {
    type Target = IdxMap<V::K, CommodityData<V>, Commodity>;

    fn deref(&self) -> &Self::Target {
        &self.idx_map
    }
}
