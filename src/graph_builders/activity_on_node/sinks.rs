use crate::graph::VIdx;
use crate::indices::IdxMap;
use crate::std_utils::Map;
use crate::time::Time;
use crate::{Problem, Variant};

pub fn create_sinks<V: Variant>(p: &Problem<V>) {
    let mut arrival_times_per_des = Map::default();
    for des in p.des_sorted_commodities.keys() {
        let mut arrival_times: IdxMap<Time, (), VIdx> = IdxMap::default();

        if let Some(ori_transports) = p.des_ori_sorted_transports.get(des) {
            for &transport in ori_transports.values().flat_map(|x| x.iter()) {
                let at = p.transport_by_idx(transport).destination().time();
                arrival_times.push_or_update(at, ());
            }
        }
        arrival_times_per_des.insert(*des, arrival_times);
    }

    // std::dbg!(arrival_times_per_des);
}
