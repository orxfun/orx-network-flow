use crate::commodities::Commodity;

pub enum ConnWaitEdge {
    Wait,
    Connect,
    Enter,
    Exit,
    Bypass(Commodity),
}
