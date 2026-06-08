use crate::graphs::graph_core::EdgeCore;
use crate::graphs::{GraphBuilderCore, VIdx, VecEdge, VecVertex, graph_core::vertex::VertexCore};

pub struct GraphCore<V, E> {
    pub(super) vertices: VecVertex<VertexCore<V>>,
    pub(super) edges: VecEdge<EdgeCore<E>>,
}

impl<V, E> GraphCore<V, E> {
    pub fn builder(vertices: impl Iterator<Item = V>) -> GraphBuilderCore<V, E> {
        GraphBuilderCore::new(vertices)
    }
}
