use crate::graphs::extended::edge::ExtEdge;
use crate::graphs::extended::graph_extended::GraphExtended;
use crate::graphs::extended::vertex::ExtVertex;
use crate::graphs::{EIdx, Graph, VIdx};

impl<'g, G, Dv, De> Graph for GraphExtended<'g, G, Dv, De>
where
    G: Graph,
{
    type Dv = Dv;

    type De = De;

    type V<'a>
        = ExtVertex<'a, G::V<'a>, Dv>
    where
        Self: 'a;

    type E<'a>
        = ExtEdge<'a, G::E<'a>, De>
    where
        Self: 'a;

    fn v(&self) -> usize {
        0
    }

    fn e(&self) -> usize {
        0
    }

    fn vertices(&self) -> impl Iterator<Item = Self::V<'_>> {
        core::iter::empty()
    }

    fn edges(&self) -> impl Iterator<Item = Self::E<'_>> {
        core::iter::empty()
    }

    fn vertex(&self, v: VIdx) -> Self::V<'_> {
        todo!()
    }

    fn edge(&self, e: EIdx) -> Self::E<'_> {
        todo!()
    }
}
