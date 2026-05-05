use crate::networks::Network;

pub struct Commodity<N, F>
where
    N: Network,
{
    s: N::NodeIdx,
    t: N::NodeIdx,
    amount: F,
}
