#[derive(Clone, Copy, Debug)]
pub struct Time(u64);

// ctors

impl From<u64> for Time {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
