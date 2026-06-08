use crate::common_ds::EitherIter;
use crate::graphs::{EIdx, Vertex, core::VertexCore};
use alloc::vec::Vec;

pub struct OriVertex<'a, Vc, D>
where
    Vc: Vertex,
{
    pub(super) core_vertex: &'a Vc,
    pub(super) ext_data: D,
    pub(super) ext_out_edges: Vec<EIdx>,
    pub(super) ext_in_edges: Vec<EIdx>,
}

pub enum ExtVertex<'a, Vc, D>
where
    Vc: Vertex,
{
    Ori(OriVertex<'a, Vc, D>),
    New(VertexCore<D>),
}

// impl<'a, Vc, D> Vertex for ExtVertex<'a, Vc, D>
// where
//     Vc: Vertex,
// {
//     type Data = D;

//     fn data(&self) -> &Self::Data {
//         match self {
//             Self::Ori(o) => &o.ext_data,
//             Self::New(v) => v.data(),
//         }
//     }

//     fn out_edges(&self) -> impl Iterator<Item = EIdx> {
//         match self {
//             Self::Ori(o) => {
//                 let ori_edges = o.core_vertex.out_edges();
//                 let ext_edges = o.ext_out_edges.iter().copied();
//                 let all_edges = ori_edges.chain(ext_edges);
//                 EitherIter::new_left(all_edges)
//             }
//             Self::New(v) => EitherIter::new_right(v.out_edges()),
//         }
//     }

//     fn in_edges(&self) -> impl Iterator<Item = EIdx> {
//         match self {
//             Self::Ori(o) => {
//                 let ori_edges = o.core_vertex.in_edges();
//                 let ext_edges = o.ext_in_edges.iter().copied();
//                 let all_edges = ori_edges.chain(ext_edges);
//                 EitherIter::new_left(all_edges)
//             }
//             Self::New(v) => EitherIter::new_right(v.in_edges()),
//         }
//     }

//     fn len_out_edges(&self) -> usize {
//         match self {
//             Self::Ori(o) => o.core_vertex.len_out_edges() + o.ext_out_edges.len(),
//             Self::New(v) => v.len_out_edges(),
//         }
//     }

//     fn len_in_edges(&self) -> usize {
//         match self {
//             Self::Ori(o) => o.core_vertex.len_in_edges() + o.ext_in_edges.len(),
//             Self::New(v) => v.len_in_edges(),
//         }
//     }
// }
