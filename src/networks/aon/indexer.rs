use crate::commodities::Commodity;
use crate::graph::VIdx;
use crate::indices::IdxCore;
use crate::networks::aon::sinks::Sinks;
use crate::transports::Transport;

pub struct Indexer {
    sinks: Sinks,
}

impl Indexer {
    pub fn new(sinks: Sinks) -> Self {
        Self { sinks }
    }

    // pub fn num_vertices(&self) -> usize {
    //     self.sinks_range.end
    // }

    // pub fn transport_idx(&self, transport: Transport) -> VIdx {
    //     let transport = transport.into_inner();
    //     let idx = self.transports_range.start + transport;
    //     debug_assert!(self.transports_range.contains(&idx));
    //     VIdx::from(idx)
    // }

    // pub fn source_idx(&self, commodity: Commodity) -> VIdx {
    //     let commodity = commodity.into_inner();
    //     let idx = self.sources_range.start + commodity;
    //     debug_assert!(self.sources_range.contains(&idx));
    //     VIdx::from(idx)
    // }

    // pub fn sink_idx(&self, commodity: Commodity) -> VIdx {
    //     let commodity = commodity.into_inner();
    //     let idx = self.sinks_range.start + commodity;
    //     debug_assert!(self.sinks_range.contains(&idx));
    //     VIdx::from(idx)
    // }
}
