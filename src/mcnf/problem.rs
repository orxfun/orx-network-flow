use crate::networks::Network;

pub struct Mcnf<N>
where
    N: Network,
{
    network: N,
}
