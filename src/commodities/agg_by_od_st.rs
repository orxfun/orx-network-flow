use crate::commodities::{Commodity, CommodityData};
use crate::{Problem, Variant};
use crate::{indices::IdxMapSubset, space_time::SpaceTimeOd, std_utils::Map};

pub struct CommoditiesByOdSt<'a, V: Variant> {
    od_commodities: Map<SpaceTimeOd, IdxMapSubset<'a, V::K, CommodityData<V>, Commodity>>,
}

impl<'a, V: Variant> CommoditiesByOdSt<'a, V> {
    pub fn create(p: &'a Problem<V>) -> Self {
        let commodities = &p.commodities.idx_map;

        let new_set = || IdxMapSubset::new(commodities);
        let mut od_commodities = Map::new();

        for (c, data) in commodities.indices_values() {
            let od_st = SpaceTimeOd::new(data.origin(), data.destination());
            od_commodities.entry(od_st).or_insert_with(new_set).push(c);
        }

        Self { od_commodities }
    }

    #[cfg(test)]
    pub(super) fn len_groups(&self) -> usize {
        self.od_commodities.len()
    }

    #[cfg(test)]
    pub(super) fn group(
        &self,
        od_st: &SpaceTimeOd,
    ) -> Option<&IdxMapSubset<'a, V::K, CommodityData<V>, Commodity>> {
        self.od_commodities.get(od_st)
    }
}
