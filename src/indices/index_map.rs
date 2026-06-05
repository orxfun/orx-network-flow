use crate::indices::IdxCore;
use crate::indices::index::Idx;
use crate::std_utils::{Map, MapKey};
use alloc::vec::Vec;
use core::fmt::Debug;
use core::marker::PhantomData;

pub struct IdxMap<K: MapKey, V, I: Idx> {
    index_and_data: Vec<(K, V)>,
    key_to_index: Map<K, usize>,
    p: PhantomData<fn() -> I>,
}

impl<K: MapKey + Debug, V: Debug, I: Idx> Debug for IdxMap<K, V, I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IdxMap")
            .field("index_and_data", &self.index_and_data)
            .field("key_to_index", &self.key_to_index)
            .finish()
    }
}

impl<K: MapKey, V, I: Idx> Default for IdxMap<K, V, I> {
    fn default() -> Self {
        Self {
            index_and_data: Default::default(),
            key_to_index: Default::default(),
            p: Default::default(),
        }
    }
}

impl<K: MapKey, V, I: Idx> FromIterator<(K, V)> for IdxMap<K, V, I> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut idx_map = Self::default();
        idx_map.extend(iter);
        idx_map
    }
}

impl<K: MapKey, V, I: Idx> Extend<(K, V)> for IdxMap<K, V, I> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, data) in iter {
            self.push_or_update(key, data);
        }
    }
}

impl<K: MapKey, V, I: Idx> IdxMap<K, V, I> {
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

    pub fn get_val_by_key(&self, key: &K) -> Option<&V> {
        let pos = *self.key_to_index.get(key)?;
        Some(&self.index_and_data[pos].1)
    }

    pub fn get_by_idx(&self, idx: I) -> Option<&V>
    where
        I: IdxCore,
    {
        let idx = idx.into_inner();
        self.index_and_data.get(idx).map(|x| &x.1)
    }

    pub fn idx_to_key(&self, idx: I) -> Option<&K>
    where
        I: IdxCore,
    {
        let idx = idx.into_inner();
        self.index_and_data.get(idx).map(|x| &x.0)
    }

    pub fn key_to_idx(&self, key: &K) -> Option<I> {
        let pos = *self.key_to_index.get(key)?;
        Some(I::from(pos))
    }

    pub fn entries(&self) -> impl Iterator<Item = (I, &K, &V)> {
        self.index_and_data
            .iter()
            .enumerate()
            .map(|(pos, (key, data))| (I::from(pos), key, data))
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.index_and_data.iter().map(|x| &x.0)
    }

    pub fn indices(&self) -> impl Iterator<Item = I> {
        (0..self.len()).map(I::from)
    }
}
