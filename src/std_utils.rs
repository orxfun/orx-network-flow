// map

#[cfg(feature = "std")]
pub type Map<K, V> = std::collections::hash_map::HashMap<K, V>;

#[cfg(not(feature = "std"))]
pub type Map<K, V> = alloc::collections::btree_map::BTreeMap<K, V>;
