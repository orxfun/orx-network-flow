use crate::commodities::Commodity;
use crate::graph::VIdx;
use crate::indices::IdxCore;
use crate::transports::Transport;
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

    pub fn transport_idx(&self, transport: Transport) -> VIdx {
        let transport = transport.into_inner();
        let idx = self.transports_range.start + transport;
        debug_assert!(self.transports_range.contains(&idx));
        VIdx::from(idx)
    }

    pub fn source_idx(&self, commodity: Commodity) -> VIdx {
        let commodity = commodity.into_inner();
        let idx = self.sources_range.start + commodity;
        debug_assert!(self.sources_range.contains(&idx));
        VIdx::from(idx)
    }

    pub fn sink_idx(&self, commodity: Commodity) -> VIdx {
        let commodity = commodity.into_inner();
        let idx = self.sinks_range.start + commodity;
        debug_assert!(self.sinks_range.contains(&idx));
        VIdx::from(idx)
    }
}
