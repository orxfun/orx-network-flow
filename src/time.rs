use core::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Time(i64);

impl Time {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn inf() -> Self {
        Time(i64::MAX)
    }

    pub(super) fn inner(self) -> i64 {
        self.0
    }
}

// From

macro_rules! impl_into_time {
    ($typename:ident) => {
        impl From<$typename> for Time {
            fn from(value: $typename) -> Self {
                Self(value as i64)
            }
        }
    };
}

impl_into_time!(i64);
impl_into_time!(u32);

// ops

impl Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Time(self.0 + rhs.0)
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Time(self.0 - rhs.0)
    }
}
