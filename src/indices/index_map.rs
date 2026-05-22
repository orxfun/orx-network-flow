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

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_by_key(&self, key: K) -> Option<&V> {
        let pos = *self.key_to_index.get(&key)?;
        Some(&self.data[pos])
    }

    pub fn entries(&self) -> impl Iterator<Item = (I, &K, &V)>
    where
        usize: From<I>,
    {
        self.key_to_index
            .iter()
            .map(|(key, pos)| ((*pos).into(), key, &self.data[*pos]))
    }
}
