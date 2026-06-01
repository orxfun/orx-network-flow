use crate::time::Time;
use core::ops::{Mul, Neg};

pub trait Cost:
    Default + Clone + Copy + Neg<Output = Self> + Mul<Self, Output = Self> + Mul<Time, Output = Self>
{
    fn zero() -> Self {
        Self::default()
    }
}

impl Cost for i64 {}

impl Mul<Time> for i64 {
    type Output = Self;

    fn mul(self, rhs: Time) -> Self::Output {
        self * rhs.inner()
    }
}
