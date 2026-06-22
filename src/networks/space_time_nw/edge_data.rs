use crate::{Commodity, Transport};

pub enum SpaceTimeEdge {
    Transport(Transport),
    Wait,
    Bypass(Commodity),
}

impl SpaceTimeEdge {
    pub fn get_transport(&self) -> Option<Transport> {
        match self {
            Self::Transport(t) => Some(*t),
            _ => None,
        }
    }

    pub fn get_bypass_c(&self) -> Option<Commodity> {
        match self {
            Self::Bypass(c) => Some(*c),
            _ => None,
        }
    }

    pub fn is_bypass(&self) -> bool {
        matches!(self, Self::Bypass(_))
    }
}
