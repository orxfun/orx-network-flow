use crate::graphs::core::{EdgeCore, GraphCoreBuilder, VertexCore};
use crate::graphs::{Edge, EdgeRange, VecEdge, VecVertex, Vertex};

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

    // transform

    pub fn map<V, E, Fv, Fe>(&self, fv: Fv, fe: Fe) -> GraphCore<V, E>
    where
        Fv: Fn(&Dv) -> V,
        Fe: Fn(&De) -> E,
    {
        let (vertices, edges) = (self.vertices.iter(), self.edges.iter());
        let vertices = vertices.map(|v| v.with_data(fv(v.data()))).collect();
        let edges = edges.map(|e| e.with_data(fe(e.data()))).collect();
        GraphCore { vertices, edges }
    }
}
