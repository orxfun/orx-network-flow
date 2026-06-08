use crate::graphs::{Edge, Graph, VIdx};

pub trait GraphMut: Graph {
    fn add_edge<'a>(&mut self, tail: VIdx, head: VIdx, data: <Self::E<'a> as Edge>::Data)
    where
        Self: 'a;
}
