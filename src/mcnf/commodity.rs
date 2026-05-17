use crate::networks::Network;

pub struct Commodity<N>
where
    N: Network,
{
    s: N::NodeIdx,
    t: N::NodeIdx,
    amount: N::Flow,
}
