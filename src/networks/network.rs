use crate::networks::Flow;

pub trait Network {
    type Flow: Flow;

    type NodeIdx;

    type EdgeIdx;

    fn edge_capacity(&self) -> Self::Flow;
}
