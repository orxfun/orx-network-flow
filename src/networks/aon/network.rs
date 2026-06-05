use crate::networks::aon::indexer::Indexer;
use crate::networks::aon::sinks::Sinks;
use crate::networks::aon::{edge::AonEdge, vertex::AonVertex};
use crate::{Graph, Problem, Variant};

pub struct AonNetwork {
    graph: Graph<AonVertex, AonEdge>,
    indexer: Indexer,
}

impl AonNetwork {
    pub fn create<V: Variant>(p: &Problem<V>) -> Self {
        let sinks = Sinks::create(p);
        todo!()
    }
}
