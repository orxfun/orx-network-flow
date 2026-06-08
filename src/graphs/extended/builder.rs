use crate::graphs::extended::edge::OriEdge;
use crate::graphs::extended::vertex::OriVertex;
use crate::graphs::extended::{ExtEdge, ExtVertex, GraphExtended};
use crate::graphs::{EIdx, Graph, VIdx, VecEdge, VecVertex};

pub struct GraphExtendedBuilder<'g, G, Dv, De>(GraphExtended<'g, G, Dv, De>)
where
    G: Graph;

impl<'g, G, Dv, De> GraphExtendedBuilder<'g, G, Dv, De>
where
    G: Graph,
{
    pub fn new(
        core: &'g G,
        core_vertices: impl IntoIterator<Item = Dv>,
        core_edges: impl IntoIterator<Item = De>,
    ) -> Self {
        let ori_vertex = |(v, data): (usize, Dv)| OriVertex {
            data,
            core_vertex: core.vertex(VIdx::from(v)),
            more_in_edges: Default::default(),
            more_out_edges: Default::default(),
        };
        let core_vertices: VecVertex<_> = core_vertices
            .into_iter()
            .enumerate()
            .map(ori_vertex)
            .collect();

        let ori_edge = |(e, data): (usize, De)| OriEdge {
            core_edge: core.edge(EIdx::from(e)),
            data,
        };
        let core_edges: VecEdge<_> = core_edges.into_iter().enumerate().map(ori_edge).collect();

        let graph = GraphExtended {
            core,
            core_vertices,
            core_edges,
            new_vertices: VecVertex::new(),
            new_edges: VecEdge::new(),
        };

        Self(graph)
    }

    pub fn edge(&mut self, data: De, tail: VIdx, head: VIdx) {}

    pub fn finish(self) -> GraphExtended<'g, G, Dv, De> {
        self.0
    }
}
