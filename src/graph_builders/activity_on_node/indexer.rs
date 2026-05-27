use crate::commodities::Commodity;
use crate::{Problem, Variant};
use crate::{graph_builders::activity_on_node::vertex::VertexData, transports::Transport};
use core::ops::Range;

pub struct Indexer {
    transports_range: Range<usize>,
    sources_range: Range<usize>,
    sinks_range: Range<usize>,
}

impl Indexer {
    pub fn new(num_commodities: usize, num_transports: usize) -> Self {
        Self {
            transports_range: 0..num_transports,
            sources_range: num_transports..(num_transports + num_commodities),
            sinks_range: (num_transports + num_commodities)..(num_transports + 2 * num_commodities),
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.sinks_range.end
    }

    pub fn vertex_data<V: Variant>(&self, prob: Problem<V>) -> impl Fn(usize) -> VertexData {
        |i| match i {
            v if self.transports_range.contains(&v) => VertexData::Transport(Transport::from(v)),
            s if self.sources_range.contains(&s) => {
                VertexData::Source(Commodity::from(s - self.transports_range.end))
            }
            t if self.sinks_range.contains(&t) => {
                VertexData::Sink(Commodity::from(t - self.sources_range.end))
            }
            _ => unreachable!(),
        }
    }
}
