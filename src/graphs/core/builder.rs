use crate::graphs::core::{GraphCore, VertexCore};
use crate::graphs::graph_mut::GraphMut;
use crate::graphs::{VIdx, VecEdge, VecVertex};

pub struct GraphCoreBuilder<Dv, De>(GraphCore<Dv, De>);

impl<Dv, De> GraphCoreBuilder<Dv, De> {
    pub fn new(vertices: impl Iterator<Item = Dv>) -> Self {
        let vertices: VecVertex<_> = vertices.map(VertexCore::new).collect();
        let edges = VecEdge::new();
        let graph = GraphCore { vertices, edges };
        Self(graph)
    }

    pub fn edge(&mut self, data: De, tail: VIdx, head: VIdx) {
        self.0.add_edge(tail, head, data);
    }

    pub fn finish(self) -> GraphCore<Dv, De> {
        self.0
    }
}
