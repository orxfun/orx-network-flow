use crate::indices::index::Index;
use crate::std_utils::{Map, MapKey};
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct IndexMap<K: MapKey, V, I: Index> {
    data: Vec<V>,
    key_to_index: Map<K, usize>,
    p: PhantomData<fn() -> I>,
}

impl<K: MapKey, V, I: Index> Default for IndexMap<K, V, I> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            key_to_index: Default::default(),
            p: Default::default(),
        }
    }
}

impl<K: MapKey, V, I: Index> IndexMap<K, V, I> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_or_update(&mut self, key: K, data: V) -> I {
        debug_assert!(!self.key_to_index.contains_key(&key));
        match self.key_to_index.get(&key) {
            Some(&pos) => {
                self.data[pos] = data;
                pos.into()
            }
            None => {
                let pos = self.data.len();
                self.data.push(data);
                self.key_to_index.insert(key, pos);
                pos.into()
            }
        }
    }
}
