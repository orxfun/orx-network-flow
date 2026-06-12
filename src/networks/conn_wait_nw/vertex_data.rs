use crate::{Commodity, SpaceTime, Transport};
use alloc::vec::Vec;

pub enum ConnWaitVertex {
    Transport(Transport),
    ReadyOri(SpaceTime, Vec<Commodity>),
    DueDes(SpaceTime, Vec<Commodity>),
}

impl ConnWaitVertex {
    pub fn get_t(&self) -> Option<Transport> {
        match self {
            Self::Transport(t) => Some(*t),
            _ => None,
        }
    }

    pub fn get_ro(&self) -> Option<(SpaceTime, &[Commodity])> {
        match self {
            Self::ReadyOri(ro, commodities) => Some((*ro, &commodities)),
            _ => None,
        }
    }

    pub fn get_dd(&self) -> Option<(SpaceTime, &[Commodity])> {
        match self {
            Self::DueDes(dd, commodities) => Some((*dd, &commodities)),
            _ => None,
        }
    }

    pub fn push_ro_commodity(&mut self, c: Commodity) -> Result<(), &str> {
        match self {
            Self::ReadyOri(_, commodities) => {
                commodities.push(c);
                Ok(())
            }
            _ => Err("push_ro_commodity called on a non-ro vertex"),
        }
    }

    pub fn push_dd_commodity(&mut self, c: Commodity) -> Result<(), &str> {
        match self {
            Self::DueDes(_, commodities) => {
                commodities.push(c);
                Ok(())
            }
            _ => Err("push_dd_commodity called on a non-dd vertex"),
        }
    }
}
