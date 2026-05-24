use crate::commodities::Commodities;
use crate::commodities::CommodityData;
use crate::problem::variant::Variant;
use crate::spaces::Spaces;

pub struct Problem<V: Variant> {
    pub(super) spaces: Spaces<V::S>,
    pub(super) commodities: Commodities<V::K>,
}

impl<V: Variant> Problem<V> {
    pub fn len_spaces(&self) -> usize {
        self.spaces.len()
    }

    pub fn len_commodities(&self) -> usize {
        self.commodities.len()
    }

    pub fn commodity(&self, key: V::K) -> Option<&CommodityData> {
        self.commodities.get_by_key(key)
    }
}
