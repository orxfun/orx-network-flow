use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Sub, SubAssign};

pub trait FlowUnit:
    Default
    + Debug
    + Clone
    + Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
    + Display
{
    fn into_f64(self) -> f64;

    fn from_f64(value: f64) -> Self;

    fn inf() -> Self;

    #[inline(always)]
    fn zero() -> Self {
        Default::default()
    }

    #[inline(always)]
    fn is_pos(self) -> bool {
        self > Self::zero()
    }

    fn sum(values: impl IntoIterator<Item = Self>) -> Self {
        let mut sum = Default::default();
        for x in values {
            sum = sum + x;
        }
        sum
    }
}
