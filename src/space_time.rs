use crate::{spaces::Space, time::Time};
use core::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SpaceTime(Space, Time);

impl SpaceTime {
    pub fn new(space: Space, time: impl Into<Time>) -> Self {
        Self(space, time.into())
    }

    pub fn space(&self) -> Space {
        self.0
    }

    pub fn time(&self) -> Time {
        self.1
    }
}

impl Display for SpaceTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, derive_new::new)]
pub struct SpaceTimeOd {
    pub ori: SpaceTime,
    pub des: SpaceTime,
}
