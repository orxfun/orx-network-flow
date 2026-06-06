use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use crate::spaces::Space;
use crate::std_utils::{Map, Set};
use crate::time::Time;
use crate::{Problem, Variant, impl_idx};
use alloc::vec::Vec;
use core::ops::Range;

impl_idx!(SourceIdx);

pub struct Sources {
    idx_map: IdxMap<SpaceTime, Time, SourceIdx>,
    ori_to_position: Map<Space, Range<usize>>,
}

impl Sources {
    pub fn create<V: Variant>(p: &Problem<V>) -> Self {
        let mut idx_map = IdxMap::default();
        let mut ori_to_position = Map::default();

        for (ori, _) in &p.ori_sorted_commodities {
            let mut departures = Set::default();

            if let Some(des_transports) = p.ori_des_sorted_transports.get(ori) {
                for &transport in des_transports.values().flat_map(|x| x.iter()) {
                    let dt = p.transport_by_idx(transport).origin().time();
                    departures.insert(dt);
                }
            }

            let mut sources: Vec<_> = departures.into_iter().collect();
            sources.sort();

            let ori_sources = sources.into_iter().map(|s| (SpaceTime::new(*ori, s), s));
            let slice_range = idx_map.len()..(idx_map.len() + ori_sources.len());
            ori_to_position.insert(*ori, slice_range);
            idx_map.extend(ori_sources);
        }

        Self {
            idx_map,
            ori_to_position,
        }
    }

    pub fn len(&self) -> usize {
        self.idx_map.len()
    }

    pub fn get_by_idx(&self, idx: SourceIdx) -> Option<Time> {
        self.idx_map.get_by_idx(idx).copied()
    }

    pub fn get_st(&self, idx: SourceIdx) -> Option<SpaceTime> {
        self.idx_map.idx_to_key(idx).copied()
    }

    pub fn get_s_idx(&self, st: SpaceTime) -> Option<SourceIdx> {
        self.idx_map.key_to_idx(&st)
    }

    pub fn iter_st_sorted(&self) -> impl Iterator<Item = SpaceTime> {
        self.idx_map.keys().copied()
    }

    pub fn slice_st_and_sources_by_ori(&self, ori: Space) -> &[(SpaceTime, Time)] {
        let idx_data_vec = self.idx_map.index_and_data();
        let slice_range = self.ori_to_position.get(&ori).expect("invalid ori");
        &idx_data_vec[slice_range.clone()]
    }

    pub fn sources_by_origins(&self) -> impl Iterator<Item = (Space, &[(SpaceTime, Time)])> {
        let idx_data_vec = self.idx_map.index_and_data();
        self.ori_to_position.iter().map(|(ori, slice_range)| {
            let sources = &idx_data_vec[slice_range.clone()];
            (*ori, sources)
        })
    }
}
