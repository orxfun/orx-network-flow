use crate::graphs::{EIdx, Edge, VIdx, core::EdgeCore};

pub struct OriEdge<'a, Ec, D>
where
    Ec: Edge,
{
    pub(super) core_edge: &'a Ec,
    pub(super) ext_data: D,
}

pub enum ExtEdge<'a, Ec, D>
where
    Ec: Edge,
{
    Ori(OriEdge<'a, Ec, D>),
    New(EdgeCore<D>),
}

impl<'a, Ec, D> Edge for ExtEdge<'a, Ec, D>
where
    Ec: Edge,
{
    type Data = D;

    fn data(&self) -> &Self::Data {
        match self {
            Self::Ori(o) => &o.ext_data,
            Self::New(n) => n.data(),
        }
    }

    fn tail(&self) -> VIdx {
        match self {
            Self::Ori(o) => o.core_edge.tail(),
            Self::New(n) => n.tail(),
        }
    }

    fn head(&self) -> VIdx {
        match self {
            Self::Ori(o) => o.core_edge.head(),
            Self::New(n) => n.head(),
        }
    }
}
