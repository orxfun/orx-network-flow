use crate::utils::std_utils::{Map, MapKey};
use alloc::vec::Vec;

pub struct SortedKeyMap<K, V>
where
    K: Ord + Clone + MapKey,
{
    map: Map<K, V>,
    sorted_keys: Vec<K>,
}

impl<K, V> Default for SortedKeyMap<K, V>
where
    K: Ord + Clone + MapKey,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            sorted_keys: Default::default(),
        }
    }
}

impl<K, V> From<Map<K, V>> for SortedKeyMap<K, V>
where
    K: Ord + Clone + MapKey,
{
    fn from(map: Map<K, V>) -> Self {
        let mut sorted_keys: Vec<_> = map.keys().cloned().collect();
        sorted_keys.sort();
        Self { map, sorted_keys }
    }
}

impl<K, V> SortedKeyMap<K, V>
where
    K: Ord + Clone + MapKey,
{
    pub fn keys(&self) -> &[K] {
        &self.sorted_keys
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        let keys = self.sorted_keys.iter();
        keys.map(|k| (k, self.map.get(k).expect("exists")))
    }

    #[inline(always)]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.map.values_mut()
    }

    #[inline]
    pub fn get_or_add_default_mut(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        if !self.map.contains_key(&key) {
            self.sorted_keys.push(key.clone());
        }
        self.map.entry(key).or_default()
    }

    // TODO: this is not nice as the struct is not always in sorted state, move to builder pattern
    pub fn preserve_key_order(&mut self) {
        self.sorted_keys.sort();
    }
}
