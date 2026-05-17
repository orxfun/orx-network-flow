use crate::{mcnf::flow::Flow, networks::Network};

pub struct Mcnf<N, F>
where
    N: Network,
    F: Flow,
{
    network: N,
}
