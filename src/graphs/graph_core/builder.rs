use crate::graphs::graph_core::{EdgeCore, VertexCore};
use crate::graphs::{EIdx, Graph, GraphCore, VIdx, VecEdge, VecVertex, Vertex};

pub struct GraphBuilderCore<V, E>(GraphCore<V, E>);

impl<V, E> GraphBuilderCore<V, E> {
    pub fn new(vertices: impl Iterator<Item = V>) -> Self {
        let vertices: VecVertex<_> = vertices.map(VertexCore::new).collect();
        let edges = VecEdge::new();
        let graph = GraphCore { vertices, edges };
        Self(graph)
    }

    pub fn edge(&mut self, data: E, tail: VIdx, head: VIdx) {
        let edges_idx = EIdx::from(self.0.edges.len());
        let tail_out_edge_idx = self.0.vertex(tail).out_edges().len();
        let head_in_edge_idx = self.0.vertex(head).in_edges().len();
        self.0.edges.push(EdgeCore::new(tail, head, data));
        self.0.vertices[tail].add_out_edge(edges_idx, head, head_in_edge_idx);
        self.0.vertices[head].add_in_edge(edges_idx, tail, tail_out_edge_idx);
    }

    pub fn finish(self) -> GraphCore<V, E> {
        self.0
    }
}
