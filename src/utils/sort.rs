use crate::utils::std_utils::{Map, Set};
use alloc::vec::Vec;
use core::hash::Hash;

pub fn map_set_into_map_sorted_vec<K, V>(map: Map<K, Set<V>>) -> Map<K, Vec<V>>
where
    K: Hash + PartialOrd + PartialEq + Eq,
    V: Ord,
{
    map.into_iter()
        .map(|(ori, ready)| {
            let mut ready: Vec<_> = ready.into_iter().collect();
            ready.sort();
            (ori, ready)
        })
        .collect()
}
