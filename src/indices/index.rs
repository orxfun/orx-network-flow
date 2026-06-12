use core::{fmt::Debug, hash::Hash};

pub trait Idx:
    Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord + From<usize> + Hash
{
}

pub trait IdxCore {
    fn into_inner(self) -> usize;
}

#[macro_export]
macro_rules! impl_idx {
    ($idx:ident, $range:ident) => {
        // idx

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

        // range
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $range(pub(super) $idx, pub(super) $idx);

        impl $range {
            pub fn new(from: $idx, to: $idx) -> Self {
                Self(from, to)
            }
        }
    };
}

impl IdxCore for usize {
    fn into_inner(self) -> usize {
        self
    }
}

impl Idx for usize {}
