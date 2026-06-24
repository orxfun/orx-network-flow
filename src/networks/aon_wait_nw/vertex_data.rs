use crate::{SpaceTime, Transport};

pub enum AonWaitVertex {
    Transport(Transport),
    ReadyOri(SpaceTime),
    DueDes(SpaceTime),
}

impl AonWaitVertex {
    pub fn get_t(&self) -> Option<Transport> {
        match self {
            Self::Transport(t) => Some(*t),
            _ => None,
        }
    }

    pub fn get_ro(&self) -> Option<SpaceTime> {
        match self {
            Self::ReadyOri(ro) => Some(*ro),
            _ => None,
        }
    }

    pub fn get_dd(&self) -> Option<(SpaceTime)> {
        match self {
            Self::DueDes(dd) => Some(*dd),
            _ => None,
        }
    }
}
