use crate::{mcnf::commodity::Commodity, networks::Network, std_utils::Map};

pub struct Mcnf<N>
where
    N: Network,
{
    network: N,
    commodities: Map<N::NodeIdx, Commodity<N>>,
}
