use core::{fmt::Debug, hash::Hash};

pub trait Idx:
    Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord + From<usize> + Hash
{
}

pub(crate) trait IdxCore {
    fn into_inner(self) -> usize;
}

#[macro_export]
macro_rules! impl_idx {
    ($idx:ident, $idx_vec:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $idx(usize);

        impl From<usize> for $idx {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }

        impl crate::indices::IdxCore for $idx {
            fn into_inner(self) -> usize {
                self.0
            }
        }

        impl crate::indices::Idx for $idx {}

        impl core::fmt::Display for $idx {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        // vec

        pub struct $idx_vec<T>(alloc::vec::Vec<T>);

        impl<T> core::ops::Index<$idx> for $idx_vec<T> {
            type Output = T;

            fn index(&self, index: $idx) -> &Self::Output {
                &self.0[index.0]
            }
        }

        impl<T> core::ops::IndexMut<$idx> for $idx_vec<T> {
            fn index_mut(&mut self, index: $idx) -> &mut Self::Output {
                &mut self.0[index.0]
            }
        }

        impl<T> $idx_vec<T> {
            pub fn new() -> Self {
                Self(Default::default())
            }

            pub fn get(&self, index: $idx) -> Option<&T> {
                self.0.get(index.0)
            }

            pub fn push(&mut self, value: T) {
                self.0.push(value);
            }
        }

        impl<T> FromIterator<T> for $idx_vec<T> {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                Self(iter.into_iter().collect())
            }
        }
    };
}
