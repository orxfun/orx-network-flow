use core::fmt::Debug;

// map

#[cfg(feature = "std")]
pub type Map<K, V> = std::collections::hash_map::HashMap<K, V>;

#[cfg(not(feature = "std"))]
pub type Map<K, V> = alloc::collections::btree_map::BTreeMap<K, V>;

// map - key

#[cfg(feature = "std")]
pub trait MapKey: Debug + Eq + core::hash::Hash {}
#[cfg(feature = "std")]
impl<K: Debug + Eq + core::hash::Hash> MapKey for K {}

#[cfg(not(feature = "std"))]
pub trait MapKey: Debug + PartialOrd + Ord {}
#[cfg(not(feature = "std"))]
impl<K: Debug + PartialOrd + Ord> MapKey for K {}

// index

#[cfg(feature = "std")]
pub trait Idx:
    Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash
{
}

#[cfg(feature = "std")]
impl<I> Idx for I where
    I: Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash
{
}

#[cfg(not(feature = "std"))]
pub trait Idx: Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord {}

#[cfg(not(feature = "std"))]
impl<I> Idx for I where I: Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord {}
