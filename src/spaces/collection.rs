use crate::Variant;
use crate::indices::IdxMap;
use crate::spaces::Space;

pub struct Spaces<V: Variant> {
    map: IdxMap<V::S, (), Space>,
}

impl<V: Variant> Default for Spaces<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<V: Variant> Spaces<V> {
    pub fn push(&mut self, key: V::S) -> Space {
        self.map.push_or_update(key, ())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn entries(&self) -> impl Iterator<Item = (Space, &V::S)> {
        self.map
            .entries()
            .into_iter()
            .map(|(space, key, _)| (space, key))
    }

    pub fn key(&self, idx: Space) -> Option<&V::S> {
        self.map.idx_to_key(idx)
    }

    pub fn get_ind_by_key(&self, key: &V::S) -> Option<Space> {
        self.map.key_to_idx(key)
    }
}
