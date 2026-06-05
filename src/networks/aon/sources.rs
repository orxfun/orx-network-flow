use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use crate::std_utils::Set;
use crate::{Problem, Variant, impl_idx};
use alloc::vec::Vec;

impl_idx!(SourceIdx);

pub struct Sources {
    idx_map: IdxMap<SpaceTime, (), SourceIdx>,
}

impl Sources {
    pub fn create<V: Variant>(p: &Problem<V>) -> Self {
        let mut departures = Set::default();

        for ori in p.ori_sorted_commodities.keys() {
            if let Some(des_transports) = p.ori_des_sorted_transports.get(ori) {
                for &transport in des_transports.values().flat_map(|x| x.iter()) {
                    let dt = p.transport_by_idx(transport).origin().time();
                    let st = SpaceTime::new(*ori, dt);
                    departures.insert(st);
                }
            }
        }
        let mut arrivals: Vec<_> = departures.into_iter().collect();
        arrivals.sort();

        let idx_map = arrivals.into_iter().map(|key| (key, ())).collect();

        Self { idx_map }
    }

    pub fn len(&self) -> usize {
        self.idx_map.len()
    }
}
