use crate::graphs::core::{EdgeCore, GraphCoreBuilder, VertexCore};
use crate::graphs::{EIdx, Edge, EdgeRange, VIdx, VecEdge, VecVertex, Vertex};

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

    // TODO: to be moved to GraphMut
    pub fn edge_data_mut(&mut self, e: EIdx) -> &mut De {
        self.edges[e].data_mut()
    }

    // transform

    pub fn map<V, E, Fv, Fe>(&self, fv: Fv, fe: Fe) -> GraphCore<V, E>
    where
        Fv: Fn(VIdx, &Dv) -> V,
        Fe: Fn(EIdx, &De) -> E,
    {
        let vertices = self.vertices.enumerated_iter();
        let vertices = vertices
            .map(|(v, x)| x.with_data(fv(v, x.data())))
            .collect();

        let edges = self.edges.enumerated_iter();
        let edges = edges.map(|(e, x)| x.with_data(fe(e, x.data()))).collect();

        GraphCore { vertices, edges }
    }
}
