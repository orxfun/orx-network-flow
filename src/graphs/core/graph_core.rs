use crate::graphs::core::{EdgeCore, GraphCoreBuilder, VertexCore};
use crate::graphs::{VecEdge, VecVertex};

pub struct GraphCore<Dv, De> {
    pub(super) vertices: VecVertex<VertexCore<Dv>>,
    pub(super) edges: VecEdge<EdgeCore<De>>,
}

impl<Dv, De> GraphCore<Dv, De> {
    pub fn builder(vertices: impl Iterator<Item = Dv>) -> GraphCoreBuilder<Dv, De> {
        GraphCoreBuilder::new(vertices)
    }
}
