#[derive(Clone, Copy, Debug)]
pub struct Time(u64);

// From

macro_rules! impl_into_time {
    ($typename:ident) => {
        impl From<$typename> for Time {
            fn from(value: $typename) -> Self {
                Self(value as u64)
            }
        }
    };
}

impl_into_time!(u64);
impl_into_time!(u32);
