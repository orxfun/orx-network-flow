use crate::graphs::core::EdgeCore;
use crate::graphs::extended::graph_extended::GraphExtended;
use crate::graphs::{EIdx, Graph, GraphMut, VIdx};

impl<'g, G, Dv, De> GraphMut for GraphExtended<'g, G, Dv, De>
where
    G: Graph,
{
    fn add_edge(&mut self, tail: VIdx, head: VIdx, data: Self::De) {
        let e = EIdx::from(self.e());

        let edge = EdgeCore::new(tail, head, data);
        self.new_edges.push(edge);

        match (self.new_v_idx(tail), self.new_v_idx(head)) {
            (Some(n1), Some(n2)) => {
                self.new_vertices[n1].add_out_edge(e);
                self.new_vertices[n2].add_in_edge(e);
            }
            (Some(n1), None) => {
                self.new_vertices[n1].add_out_edge(e);
                self.core_vertices[head].more_in_edges.push(e);
            }
            (None, Some(n2)) => {
                self.core_vertices[tail].more_out_edges.push(e);
                self.new_vertices[n2].add_in_edge(e);
            }
            (None, None) => {
                self.core_vertices[tail].more_out_edges.push(e);
                self.core_vertices[head].more_in_edges.push(e);
            }
        }
    }
}
