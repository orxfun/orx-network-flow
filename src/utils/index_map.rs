use crate::std_utils::{Map, MapKey};
use alloc::vec::Vec;

pub struct IndexMap<K: MapKey, V> {
    data: Vec<V>,
    key_to_index: Map<K, usize>,
}
