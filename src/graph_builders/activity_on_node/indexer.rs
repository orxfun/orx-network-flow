use crate::{Problem, Variant, graph_builders::activity_on_node::vertex::VertexData};

pub struct Indexer {
    num_commodities: usize,
    num_transports: usize,
}

impl Indexer {
    pub fn new(num_commodities: usize, num_transports: usize) -> Self {
        Self {
            num_commodities,
            num_transports,
        }
    }

    pub fn num_vertices(&self) -> usize {
        2 * self.num_commodities + self.num_transports
    }

    pub fn vertex_data<V: Variant>(&self, prob: Problem<V>) -> impl Fn(usize) -> VertexData {
        |i| VertexData::Sink(todo!())
    }
}
