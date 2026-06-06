use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use crate::spaces::Space;
use crate::std_utils::{Map, Set};
use crate::time::Time;
use crate::{Problem, Variant, impl_idx};
use alloc::vec::Vec;
use core::ops::Range;

impl_idx!(SinkIdx);

pub struct Sinks {
    idx_map: IdxMap<SpaceTime, Time, SinkIdx>,
    des_to_position: Map<Space, Range<usize>>,
}

impl Sinks {
    pub fn create<V: Variant>(p: &Problem<V>) -> Self {
        let mut idx_map = IdxMap::default();
        let mut des_to_position = Map::default();

        for (des, _) in &p.des_sorted_commodities {
            let mut arrivals = Set::default();

            if let Some(ori_transports) = p.des_ori_sorted_transports.get(des) {
                for &transport in ori_transports.values().flat_map(|x| x.iter()) {
                    let at = p.transport_by_idx(transport).destination().time();
                    arrivals.insert(at);
                }
            }

            let mut sinks: Vec<_> = arrivals.into_iter().collect();
            sinks.sort();

            let des_sinks = sinks.into_iter().map(|t| (SpaceTime::new(*des, t), t));
            let slice_range = idx_map.len()..(idx_map.len() + des_sinks.len());
            des_to_position.insert(*des, slice_range);
            idx_map.extend(des_sinks);
        }

        Self {
            idx_map,
            des_to_position,
        }
    }

    pub fn len(&self) -> usize {
        self.idx_map.len()
    }

    pub fn get_by_idx(&self, idx: SinkIdx) -> Option<Time> {
        self.idx_map.get_by_idx(idx).copied()
    }

    pub fn get_st(&self, idx: SinkIdx) -> Option<SpaceTime> {
        self.idx_map.idx_to_key(idx).copied()
    }

    pub fn get_t_idx(&self, st: SpaceTime) -> Option<SinkIdx> {
        self.idx_map.key_to_idx(&st)
    }

    pub fn iter_st_sorted(&self) -> impl Iterator<Item = SpaceTime> {
        self.idx_map.keys().copied()
    }

    pub fn chunks_by_destinations(&self) -> impl Iterator<Item = (Space, &[(SpaceTime, Time)])> {
        let idx_data_vec = self.idx_map.index_and_data();
        self.des_to_position.iter().map(|(ori, slice_range)| {
            let sinks = &idx_data_vec[slice_range.clone()];
            (*ori, sinks)
        })
    }
}
