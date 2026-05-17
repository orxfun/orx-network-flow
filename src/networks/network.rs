use crate::{networks::Flow, std_utils::Idx};

pub trait Network {
    type Flow: Flow;

    type NodeIdx: Idx;

    type EdgeIdx: Idx;

    fn edge_capacity(&self) -> Self::Flow;
}
