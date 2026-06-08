use crate::graphs::core::{GraphCore, VertexCore};
use crate::graphs::graph_mut::GraphMut;
use crate::graphs::{Graph, VIdx, VecEdge, VecVertex};

pub struct GraphCoreBuilder<Dv, De>(GraphCore<Dv, De>);

impl<Dv, De> GraphCoreBuilder<Dv, De> {
    pub fn new() -> Self {
        let vertices = VecVertex::new();
        let edges = VecEdge::new();
        let graph = GraphCore { vertices, edges };
        Self(graph)
    }

    pub fn node(&mut self, data: Dv) -> VIdx {
        let idx = VIdx::from(self.0.v());
        let vertex = VertexCore::new(data);
        self.0.vertices.push(vertex);
        idx
    }

    pub fn edge(&mut self, data: De, tail: VIdx, head: VIdx) {
        self.0.add_edge(tail, head, data);
    }

    pub fn finish(self) -> GraphCore<Dv, De> {
        self.0
    }
}
