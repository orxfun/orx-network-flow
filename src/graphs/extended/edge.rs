use crate::graphs::{Edge, VIdx, core::EdgeCore};

pub struct OriEdge<'a, Ec, De>
where
    Ec: Edge,
{
    pub(super) core_edge: &'a Ec,
    pub(super) data: De,
}

pub enum ExtEdge<'a, Ec, De>
where
    Ec: Edge,
{
    Ori(OriEdge<'a, Ec, De>),
    New(&'a EdgeCore<De>),
}

impl<'a, Ec, De> Edge for ExtEdge<'a, Ec, De>
where
    Ec: Edge,
{
    type Data = De;

    fn data(&self) -> &Self::Data {
        match self {
            Self::Ori(o) => &o.data,
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
