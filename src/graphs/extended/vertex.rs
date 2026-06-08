use crate::common_ds::EitherIter;
use crate::graphs::{EIdx, Vertex, core::VertexCore};
use alloc::vec::Vec;

pub struct OriVertex<'a, Vc, Dv>
where
    Vc: Vertex,
{
    pub(super) core_vertex: &'a Vc,
    pub(super) data: Dv,
    pub(super) more_out_edges: Vec<EIdx>,
    pub(super) more_in_edges: Vec<EIdx>,
}

pub enum ExtVertex<'a, 'g, Vc, Dv>
where
    Vc: Vertex,
{
    Ori(&'a OriVertex<'g, Vc, Dv>),
    New(&'a VertexCore<Dv>),
}

impl<'a, 'g, Vc, Dv> Vertex for ExtVertex<'a, 'g, Vc, Dv>
where
    Vc: Vertex + 'g,
{
    type Data = Dv;

    fn data(&self) -> &Self::Data {
        match self {
            Self::Ori(o) => &o.data,
            Self::New(v) => v.data(),
        }
    }

    fn out_edges(&self) -> impl Iterator<Item = EIdx> {
        match self {
            Self::Ori(o) => {
                let ori_edges = o.core_vertex.out_edges();
                let ext_edges = o.more_out_edges.iter().copied();
                let all_edges = ori_edges.chain(ext_edges);
                EitherIter::new_left(all_edges)
            }
            Self::New(v) => EitherIter::new_right(v.out_edges()),
        }
    }

    fn in_edges(&self) -> impl Iterator<Item = EIdx> {
        match self {
            Self::Ori(o) => {
                let ori_edges = o.core_vertex.in_edges();
                let ext_edges = o.more_in_edges.iter().copied();
                let all_edges = ori_edges.chain(ext_edges);
                EitherIter::new_left(all_edges)
            }
            Self::New(v) => EitherIter::new_right(v.in_edges()),
        }
    }

    fn len_out_edges(&self) -> usize {
        match self {
            Self::Ori(o) => o.core_vertex.len_out_edges() + o.more_out_edges.len(),
            Self::New(v) => v.len_out_edges(),
        }
    }

    fn len_in_edges(&self) -> usize {
        match self {
            Self::Ori(o) => o.core_vertex.len_in_edges() + o.more_in_edges.len(),
            Self::New(v) => v.len_in_edges(),
        }
    }
}
