use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use crate::std_utils::Set;
use crate::{Problem, Variant, impl_idx};
use alloc::vec::Vec;

impl_idx!(SinkIdx);

pub struct Sinks {
    idx_map: IdxMap<SpaceTime, (), SinkIdx>,
}

impl Sinks {
    pub fn create<V: Variant>(p: &Problem<V>) -> Self {
        let mut arrivals = Set::default();

        for des in p.des_sorted_commodities.keys() {
            if let Some(ori_transports) = p.des_ori_sorted_transports.get(des) {
                for &transport in ori_transports.values().flat_map(|x| x.iter()) {
                    let at = p.transport_by_idx(transport).destination().time();
                    let st = SpaceTime::new(*des, at);
                    arrivals.insert(st);
                }
            }
        }
        let mut arrivals: Vec<_> = arrivals.into_iter().collect();
        arrivals.sort();

        let idx_map = arrivals.into_iter().map(|key| (key, ())).collect();

        Self { idx_map }
    }

    pub fn len(&self) -> usize {
        self.idx_map.len()
    }

    pub fn get_st(&self, idx: SinkIdx) -> Option<SpaceTime> {
        self.idx_map.idx_to_key(idx).copied()
    }
}
