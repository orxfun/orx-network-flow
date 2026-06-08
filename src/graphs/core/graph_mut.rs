use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::graph_mut::GraphMut;
use crate::graphs::{EIdx, Edge, Graph, VIdx};

impl<V, E> GraphMut for GraphCore<V, E> {
    fn add_edge(&mut self, tail: VIdx, head: VIdx, data: <Self::E as Edge>::Data) {
        let e = EIdx::from(self.e());

        let edge = EdgeCore::new(tail, head, data);
        self.edges.push(edge);

        self.vertices[tail].add_out_edge(e);
        self.vertices[head].add_in_edge(e);
    }
}
