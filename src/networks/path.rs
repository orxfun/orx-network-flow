use crate::networks::Network;
use alloc::vec::Vec;

pub struct Path<N: Network> {
    vertices: Vec<N::NodeIdx>,
    edges: Vec<N::EdgeIdx>,
}
