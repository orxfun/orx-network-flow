use crate::commodities::Commodity;

pub enum ConnWaitEdge {
    Wait,
    Connect,
    Enter,
    Exit,
    Bypass(Commodity),
}

impl ConnWaitEdge {
    pub fn get_bypass_c(&self) -> Option<Commodity> {
        match self {
            Self::Bypass(c) => Some(*c),
            _ => None,
        }
    }
}
