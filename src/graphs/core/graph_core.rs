use crate::graphs::core::visualization::DotGraphCore;
use crate::graphs::core::{EdgeCore, GraphCoreBuilder, VertexCore};
use crate::graphs::visualization::dot::NodeSettings;
use crate::graphs::{VecEdge, VecVertex};

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
