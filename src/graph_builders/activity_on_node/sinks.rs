use crate::graph::VIdx;
use crate::indices::IdxMap;
use crate::space_time::SpaceTime;
use crate::spaces::Space;
use crate::std_utils::{Map, Set};
use crate::time::Time;
use crate::{Problem, Variant};
use alloc::vec::Vec;

pub struct Sinks {
    idx_map: IdxMap<SpaceTime, (), VIdx>,
}

pub fn create_sinks<V: Variant>(p: &Problem<V>, num_vertices_before: usize) -> Sinks {
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

    std::dbg!(&idx_map);

    Sinks { idx_map }
}
