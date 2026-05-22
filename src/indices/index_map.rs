use crate::indices::index::Index;
use crate::std_utils::{Map, MapKey};
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct IndexMap<K: MapKey, V, I: Index> {
    index_and_data: Vec<(K, V)>,
    key_to_index: Map<K, usize>,
    p: PhantomData<fn() -> I>,
}

impl<K: MapKey, V, I: Index> Default for IndexMap<K, V, I> {
    fn default() -> Self {
        Self {
            index_and_data: Default::default(),
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
                debug_assert_eq!(&self.index_and_data[pos].0, &key);
                self.index_and_data[pos].1 = data;
                pos.into()
            }
            None => {
                let pos = self.index_and_data.len();
                self.index_and_data.push((key.clone(), data));
                self.key_to_index.insert(key, pos);
                pos.into()
            }
        }
    }

    pub fn len(&self) -> usize {
        self.index_and_data.len()
    }

    pub fn get_by_key(&self, key: K) -> Option<&V> {
        let pos = *self.key_to_index.get(&key)?;
        Some(&self.index_and_data[pos].1)
    }

    pub fn entries(&self) -> impl Iterator<Item = (I, &K, &V)>
    where
        usize: From<I>,
    {
        // self.key_to_index
        //     .iter()
        //     .map(|(key, pos)| ((*pos).into(), key, &self.index_and_data[*pos]))

        core::iter::empty()
    }
}
