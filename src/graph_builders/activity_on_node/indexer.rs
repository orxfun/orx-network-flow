use crate::commodities::Commodity;
use crate::{Problem, Variant};
use crate::{graph_builders::activity_on_node::vertex::VertexData, transports::Transport};
use core::iter;
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
}
