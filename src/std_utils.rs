use core::fmt::Debug;
#[cfg(feature = "std")]
use core::fmt::Display;

// set

#[cfg(feature = "std")]
pub type Set<K> = std::collections::hash_set::HashSet<K>;

#[cfg(not(feature = "std"))]
pub type Set<K> = alloc::collections::btree_set::BTreeSet<K>;

// map

#[cfg(feature = "std")]
pub type Map<K, V> = std::collections::hash_map::HashMap<K, V>;

#[cfg(not(feature = "std"))]
pub type Map<K, V> = alloc::collections::btree_map::BTreeMap<K, V>;

// map - key

#[cfg(feature = "std")]
pub trait MapKey: Debug + Display + Clone + Eq + core::hash::Hash {}
#[cfg(feature = "std")]
impl<K: Debug + Display + Clone + Eq + core::hash::Hash> MapKey for K {}

#[cfg(not(feature = "std"))]
pub trait MapKey: Debug + Display + Clone + PartialOrd + Ord {}
#[cfg(not(feature = "std"))]
impl<K: Debug + Display + Clone + PartialOrd + Ord> MapKey for K {}
