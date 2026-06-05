use crate::Graph;
use crate::networks::aon::indexer::Indexer;
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};

pub struct AonNetwork {
    graph: Graph<AonVertex, AonEdge>,
    indexer: Indexer,
}
