use crate::commodities::Commodity;
use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use crate::spaces::Space;
use crate::std_utils::{Map, Set};
use crate::time::Time;
use crate::{Problem, Variant, impl_idx};
use alloc::vec::Vec;

impl_idx!(SourceIdx);

// pub struct Sources {
//     idx_map: IdxMap<SpaceTime, (), SourceIdx>,
// }

// impl Sources {
//     pub fn create<V: Variant>(p: &Problem<V>) -> Self {
//         let mut departures = Set::default();

//         for ori in p.ori_sorted_commodities.keys() {
//             if let Some(des_transports) = p.ori_des_sorted_transports.get(ori) {
//                 for &transport in des_transports.values().flat_map(|x| x.iter()) {
//                     let dt = p.transport_by_idx(transport).origin().time();
//                     let st = SpaceTime::new(*ori, dt);
//                     departures.insert(st);
//                 }
//             }
//         }
//         let mut departures: Vec<_> = departures.into_iter().collect();
//         departures.sort();

//         let idx_map = departures.into_iter().map(|key| (key, ())).collect();

//         Self { idx_map }
//     }

//     pub fn len(&self) -> usize {
//         self.idx_map.len()
//     }

//     pub fn get_st(&self, idx: SourceIdx) -> Option<SpaceTime> {
//         self.idx_map.idx_to_key(idx).copied()
//     }

//     pub fn get_s_idx(&self, st: SpaceTime) -> Option<SourceIdx> {
//         self.idx_map.key_to_idx(&st)
//     }

//     pub fn iter_st_sorted(&self) -> impl Iterator<Item = SpaceTime> {
//         self.idx_map.keys().copied()
//     }
// }

pub struct Sources {
    idx_map: IdxMap<SpaceTime, Source, SourceIdx>,
    no_source_commodities: Vec<Commodity>,
}

impl Sources {
    pub fn create<V: Variant>(p: &Problem<V>) -> Self {
        let mut no_source_commodities = Vec::new();
        let mut idx_map = IdxMap::default();

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
                    None => no_source_commodities.push(c),
                }
            }

            let ori_sources = sources.into_iter().map(|s| (SpaceTime::new(*ori, s.dt), s));
            idx_map.extend(ori_sources);
        }

        Self {
            idx_map,
            no_source_commodities,
        }
    }

    pub fn len(&self) -> usize {
        self.idx_map.len()
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
