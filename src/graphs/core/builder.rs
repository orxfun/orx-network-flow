use crate::graphs::core::{EdgeCore, VertexCore};
use crate::graphs::{EIdx, GraphCore, VIdx, VecEdge, VecVertex};

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
        self.0.edges.push(EdgeCore::new(tail, head, data));
        self.0.vertices[tail].add_out_edge(edges_idx);
        self.0.vertices[head].add_in_edge(edges_idx);
    }

    pub fn finish(self) -> GraphCore<V, E> {
        self.0
    }
}
