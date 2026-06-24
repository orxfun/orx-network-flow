use crate::commodities::Commodity;

pub enum AonWaitEdge {
    Wait,
    Connect,
    Enter,
    Exit,
    Bypass(Commodity),
}

impl AonWaitEdge {
    pub fn get_bypass_c(&self) -> Option<Commodity> {
        match self {
            Self::Bypass(c) => Some(*c),
            _ => None,
        }
    }
}
