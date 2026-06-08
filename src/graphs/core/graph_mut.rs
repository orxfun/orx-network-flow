use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::{EIdx, Graph, GraphMut, VIdx};

impl<Dv, De> GraphMut for GraphCore<Dv, De> {
    fn add_edge(&mut self, tail: VIdx, head: VIdx, data: Self::De) {
        let e = EIdx::from(self.e());

        let edge = EdgeCore::new(tail, head, data);
        self.edges.push(edge);

        self.vertices[tail].add_out_edge(e);
        self.vertices[head].add_in_edge(e);
    }
}
