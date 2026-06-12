use crate::{commodities::Commodity, transports::Transport};

pub enum ConnWaitEdge {
    Wait,
    Connect(Transport),
    Enter,
    Exit(Transport),
    Bypass(Commodity),
}
