use crate::graphs::extended::vertex::OriVertex;
use crate::graphs::extended::{ExtEdge, ExtVertex, GraphExtended};
use crate::graphs::{EIdx, Graph, VIdx, VecVertex};

pub struct GraphExtendedBuilder<'g, G, Dv, De>(GraphExtended<'g, G, Dv, De>)
where
    G: Graph;

// impl<'g, G, Dv, De> GraphExtendedBuilder<'g, G, Dv, De>
// where
//     G: Graph,
// {
//     pub fn new(
//         core_graph: &'g G,
//         core_vertices: impl IntoIterator<Item = Dv>,
//         core_edges: impl IntoIterator<Item = De>,
//     ) -> Self {
//         let ori_vertex = |i: usize, data: Dv| OriVertex {
//             data,
//             core_vertex: core_graph.vertex(VIdx::from(i)),
//             more_in_edges: Default::default(),
//             more_out_edges: Default::default(),
//         };
//         // let core_vertices :VecVertex<_> = core_vertices.into_iter()
//         //     .map(|data|)
//     }

//     pub fn edge(&mut self, data: De, tail: VIdx, head: VIdx) {}

//     pub fn finish(self) -> GraphExtended<'g, G, Dv, De> {
//         self.0
//     }
// }
