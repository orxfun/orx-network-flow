use crate::commodities::Commodity;
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
    idx_map: IdxMap<SpaceTime, Source, SourceIdx>,
    ori_to_position: Map<Space, Range<usize>>,
}

impl Sources {
    pub fn create<V: Variant>(p: &Problem<V>) -> (Self, Set<Commodity>) {
        let mut no_source_commodities = Set::default();
        let mut idx_map = IdxMap::default();
        let mut ori_to_position = Map::default();

        for (ori, sorted_commodities) in &p.ori_sorted_commodities {
            let mut departures = Set::default();

            if let Some(des_transports) = p.ori_des_sorted_transports.get(ori) {
                for &transport in des_transports.values().flat_map(|x| x.iter()) {
                    let dt = p.transport_by_idx(transport).origin().time();
                    departures.insert(dt);
                }
            }

            let mut sources: Vec<_> = departures.into_iter().map(Source::new).collect();
            sources.sort_by_key(|s| s.dt);

            for &c in sorted_commodities {
                let ready = p.commodity_by_idx(c).origin().time();
                let max_waiting = p.time_bounds.max_waiting.bound(p, c);
                let max_dt = ready + max_waiting;
                // TODO: might use binary search here
                match sources.iter().position(|s| s.dt >= ready && s.dt <= max_dt) {
                    Some(s) => sources[s].commodities.push(c),
                    None => _ = no_source_commodities.insert(c),
                }
            }

            let ori_sources = sources.into_iter().map(|s| (SpaceTime::new(*ori, s.dt), s));
            let slice_range = idx_map.len()..(idx_map.len() + ori_sources.len());
            ori_to_position.insert(*ori, slice_range);
            idx_map.extend(ori_sources);
        }

        let sources = Self {
            idx_map,
            ori_to_position,
        };
        (sources, no_source_commodities)
    }

    pub fn len(&self) -> usize {
        self.idx_map.len()
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

    pub fn iter_sidx_and_commodities(&self) -> impl Iterator<Item = (SourceIdx, &[Commodity])> {
        self.idx_map
            .entries()
            .map(|(sidx, _, source)| (sidx, source.commodities.as_slice()))
    }

    pub fn slice_st_and_sources_by_ori(&self, ori: Space) -> &[(SpaceTime, Source)] {
        let idx_data_vec = self.idx_map.index_and_data();
        let slice_range = self.ori_to_position.get(&ori).expect("invalid ori");
        &idx_data_vec[slice_range.clone()]
    }

    pub fn sources_by_origins(&self) -> impl Iterator<Item = (Space, &[(SpaceTime, Source)])> {
        let idx_data_vec = self.idx_map.index_and_data();
        self.ori_to_position.iter().map(|(ori, slice_range)| {
            let sources = &idx_data_vec[slice_range.clone()];
            (*ori, sources)
        })
    }
}

pub struct Source {
    dt: Time,
    commodities: Vec<Commodity>,
}

impl Source {
    fn new(dt: Time) -> Self {
        Self {
            dt,
            commodities: Default::default(),
        }
    }
}
