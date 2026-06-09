use crate::indices::{Idx, IdxCore, IdxMap};
use crate::std_utils::MapKey;
use alloc::vec::Vec;

pub struct IdxMapSubset<'a, K: MapKey, V, I: Idx> {
    map: &'a IdxMap<K, V, I>,
    subset: Vec<I>,
}

impl<'a, K: MapKey, V, I: Idx> IdxMapSubset<'a, K, V, I> {
    pub fn new(map: &'a IdxMap<K, V, I>) -> Self {
        let subset = Vec::new();
        Self { map, subset }
    }

    pub fn push(&mut self, idx: I) {
        self.subset.push(idx);
    }

    pub fn len(&self) -> usize {
        self.subset.len()
    }

    pub fn indices(&self) -> impl Iterator<Item = I> {
        self.subset.iter().copied()
    }

    pub fn indices_values(&self) -> impl Iterator<Item = (I, &V)>
    where
        I: IdxCore,
    {
        self.subset
            .iter()
            .copied()
            .map(|idx| (idx, self.map.value_unchecked(idx)))
    }

    pub fn values(&self) -> impl Iterator<Item = &V>
    where
        I: IdxCore,
    {
        self.subset
            .iter()
            .copied()
            .map(|idx| self.map.value_unchecked(idx))
    }
}
