use crate::{mcnf::commodity::Commodity, networks::Network};
use alloc::vec::Vec;

pub struct Mcnf<N>
where
    N: Network,
{
    network: N,
    commodities: Vec<Commodity<N>>,
}
