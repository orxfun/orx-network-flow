use core::fmt::Debug;

pub trait Index:
    Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord + From<usize>
{
}

#[macro_export]
macro_rules! impl_from_usize {
    ($typename:ident) => {
        impl From<usize> for $typename {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
    };
}
