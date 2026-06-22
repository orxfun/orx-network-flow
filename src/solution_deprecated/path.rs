use crate::Transport;
use alloc::vec::Vec;

pub enum Path {
    OneLeg(Transport),
    TwoLegs([Transport; 2]),
    ThreeLegs([Transport; 3]),
    Long(Vec<Transport>),
}
