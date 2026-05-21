#[derive(Clone, Copy, Debug)]
pub struct Time(u64);

// ctors

impl From<u64> for Time {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<u32> for Time {
    fn from(value: u32) -> Self {
        Self(value as u64)
    }
}
