use crate::graphs::core::EdgeCore;
use crate::graphs::core::visualization::DotGraphCore;
use crate::graphs::visualization::dot::NodeSettings;
use crate::graphs::{GraphCoreBuilder, VecEdge, VecVertex, core::vertex::VertexCore};

pub struct GraphCore<Dv, De> {
    pub(super) vertices: VecVertex<VertexCore<Dv>>,
    pub(super) edges: VecEdge<EdgeCore<De>>,
}

impl<Dv, De> GraphCore<Dv, De> {
    pub fn builder(vertices: impl Iterator<Item = Dv>) -> GraphCoreBuilder<Dv, De> {
        GraphCoreBuilder::new(vertices)
    }

    pub fn dot_graph(&self, settings: Option<NodeSettings>) -> DotGraphCore<'_, Dv, De> {
        DotGraphCore::new(self, settings.unwrap_or_default())
    }
}
