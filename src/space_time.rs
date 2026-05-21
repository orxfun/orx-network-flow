use crate::{space::Space, time::Time};

#[derive(Clone, Copy, Debug)]
pub struct SpaceTime(Space, Time);

impl SpaceTime {
    pub fn new(space: Space, time: impl Into<Time>) -> Self {
        Self(space, time.into())
    }
}
