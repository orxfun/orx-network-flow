use crate::graphs::{Edge, Graph, VIdx};

pub trait GraphMut: Graph {
    fn add_edge(&mut self, tail: VIdx, head: VIdx, data: <Self::E as Edge>::Data);
}
