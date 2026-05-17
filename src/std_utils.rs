use core::fmt::Debug;

// map

#[cfg(feature = "std")]
pub type Map<K, V> = std::collections::hash_map::HashMap<K, V>;

#[cfg(not(feature = "std"))]
pub type Map<K, V> = alloc::collections::btree_map::BTreeMap<K, V>;

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
