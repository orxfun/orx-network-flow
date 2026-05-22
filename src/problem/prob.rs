use crate::commodities::{Commodity, CommodityData};
use crate::space_time::SpaceTime;
use crate::spaces::Spaces;
use crate::time::Time;
use crate::{commodities::Commodities, std_utils::MapKey};

pub struct Problem<S, K>
where
    S: MapKey,
    K: MapKey,
{
    pub(super) spaces: Spaces<S>,
    pub(super) commodities: Commodities<K>,
}

impl<S, K> Problem<S, K>
where
    S: MapKey,
    K: MapKey,
{
    pub fn len_spaces(&self) -> usize {
        self.spaces.len()
    }

    pub fn len_commodities(&self) -> usize {
        self.commodities.len()
    }

    pub fn commodity(&self, key: K) -> Option<&CommodityData> {
        self.commodities.get_by_key(key)
    }
}
