use core::fmt::Debug;

pub trait Index:
    Debug + Clone + Copy + Send + Sync + PartialEq + Eq + PartialOrd + Ord + From<usize>
{
}
