use crate::graphs::graph_core::{EdgeCore, GraphCore};
use crate::graphs::graph_mut::GraphMut;
use crate::graphs::{EIdx, Edge, Graph, VIdx, Vertex};

impl<V, E> GraphMut for GraphCore<V, E> {
    fn add_edge<'a>(&mut self, tail: VIdx, head: VIdx, data: <Self::E<'a> as Edge>::Data)
    where
        Self: 'a,
    {
        let e = EIdx::from(self.e());

        let tail_out_edge_pos = self.vertex(tail).len_out_edges();
        let head_in_edge_pos = self.vertex(head).len_in_edges();

        let edge = EdgeCore::new(tail, head, data);
        self.edges.push(edge);

        self.vertices[tail].add_out_edge(e, head, head_in_edge_pos);
        self.vertices[head].add_in_edge(e, tail, tail_out_edge_pos);
    }
}
