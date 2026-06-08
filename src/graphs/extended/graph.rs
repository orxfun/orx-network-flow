use crate::graphs::extended::{ExtEdge, ExtVertex, GraphExtended};
use crate::graphs::{EIdx, Graph, VIdx};

impl<'g, G, Dv, De> Graph for GraphExtended<'g, G, Dv, De>
where
    G: Graph,
{
    type Dv = Dv;

    type De = De;

    type V<'a>
        = ExtVertex<'a, 'g, G::V<'g>, Dv>
    where
        Self: 'a;

    type E<'a>
        = ExtEdge<'a, G::E<'g>, De>
    where
        Self: 'a;

    fn v(&self) -> usize {
        self.core.v() + self.new_vertices.len()
    }

    fn e(&self) -> usize {
        self.core.e() + self.new_edges.len()
    }

    fn vertices(&self) -> impl Iterator<Item = Self::V<'_>> {
        core::iter::empty()
    }

    fn edges(&self) -> impl Iterator<Item = Self::E<'_>> {
        core::iter::empty()
    }

    fn vertex<'a>(&'a self, v: VIdx) -> Self::V<'a>
    where
        Self: 'a,
    {
        match self.new_v_idx(v) {
            None => ExtVertex::Ori(&self.core_vertices[v]),
            Some(n) => ExtVertex::New(&self.new_vertices[n]),
        }
    }

    fn edge<'a>(&'a self, e: EIdx) -> Self::E<'a>
    where
        Self: 'a,
    {
        match self.new_e_idx(e) {
            None => ExtEdge::Ori(&self.core_edges[e]),
            Some(n) => ExtEdge::New(&self.new_edges[n]),
        }
    }
}
