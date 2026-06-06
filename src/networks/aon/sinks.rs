use crate::commodities::Commodity;
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
    idx_map: IdxMap<SpaceTime, Sink, SinkIdx>,
    des_to_position: Map<Space, Range<usize>>,
}

impl Sinks {
    pub fn create<V: Variant>(p: &Problem<V>) -> (Self, Set<Commodity>) {
        let mut no_sink_commodities = Set::default();
        let mut idx_map = IdxMap::default();
        let mut des_to_position = Map::default();

        for (des, sorted_commodities) in &p.des_sorted_commodities {
            let mut arrivals = Set::default();

            if let Some(ori_transports) = p.des_ori_sorted_transports.get(des) {
                for &transport in ori_transports.values().flat_map(|x| x.iter()) {
                    let at = p.transport_by_idx(transport).destination().time();
                    arrivals.insert(at);
                }
            }

            let mut sinks: Vec<_> = arrivals.into_iter().map(Sink::new).collect();
            sinks.sort_by_key(|s| s.at);

            for &c in sorted_commodities {
                let due = p.commodity_by_idx(c).destination().time();
                let max_earliness = p.time_bounds.max_earliness.bound(p, c);
                let min_at = due - max_earliness;
                // TODO: might use binary search here
                let commodity_can_exit = |t: &&mut Sink| t.at >= min_at && t.at <= due;
                let fitting_sinks = sinks.iter_mut().filter(commodity_can_exit);
                let mut any_fitting_sink = false;
                for sink in fitting_sinks {
                    sink.commodities.push(c);
                    any_fitting_sink = true;
                }

                if !any_fitting_sink {
                    no_sink_commodities.insert(c);
                }
            }

            let num_sinks = sinks.iter().filter(|s| !s.commodities.is_empty()).count();
            let des_sinks = sinks
                .into_iter()
                .filter(|s| !s.commodities.is_empty())
                .map(|t| (SpaceTime::new(*des, t.at), t));
            let slice_range = idx_map.len()..(idx_map.len() + num_sinks);
            des_to_position.insert(*des, slice_range);
            idx_map.extend(des_sinks);
        }

        let sinks = Self {
            idx_map,
            des_to_position,
        };
        (sinks, no_sink_commodities)
    }

    pub fn len(&self) -> usize {
        self.idx_map.len()
    }

    pub fn get_by_idx(&self, idx: SinkIdx) -> Option<&Sink> {
        self.idx_map.get_by_idx(idx)
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

    pub fn iter_tidx_and_commodities(&self) -> impl Iterator<Item = (SinkIdx, &[Commodity])> {
        self.idx_map
            .entries()
            .map(|(sidx, _, sink)| (sidx, sink.commodities.as_slice()))
    }
}

pub struct Sink {
    at: Time,
    commodities: Vec<Commodity>,
}

impl Sink {
    fn new(at: Time) -> Self {
        Self {
            at,
            commodities: Default::default(),
        }
    }

    pub fn commodities(&self) -> &[Commodity] {
        &self.commodities
    }
}
