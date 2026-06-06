use core::fmt::{Debug, Display};
use core::ops::Add;

pub trait FlowUnit:
    Default + Debug + Clone + Copy + PartialOrd + Add<Output = Self> + Display
{
    fn sum(values: impl IntoIterator<Item = Self>) -> Self {
        let mut sum = Default::default();
        for x in values {
            sum = sum + x;
        }
        sum
    }
}
