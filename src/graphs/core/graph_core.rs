use crate::graphs::core::{EdgeCore, GraphCoreBuilder, VertexCore};
use crate::graphs::{EdgeRange, VecEdge, VecVertex};

pub struct GraphCore<Dv, De> {
    pub(super) vertices: VecVertex<VertexCore<Dv>>,
    pub(super) edges: VecEdge<EdgeCore<De>>,
}

impl<Dv, De> GraphCore<Dv, De> {
    pub fn builder() -> GraphCoreBuilder<Dv, De> {
        GraphCoreBuilder::new()
    }

    pub fn edges_slice(&self, range: EdgeRange) -> &[EdgeCore<De>] {
        self.edges.slice(range)
    }
}
