use crate::commodities::Commodity;
use crate::graph::VIdx;
use crate::indices::IdxCore;
use crate::networks::aon::sinks::Sinks;
use crate::networks::aon::sources::Sources;
use crate::transports::Transport;

pub struct Indexer {
    len_transports: usize,
    sources: Sources,
    sinks: Sinks,
}

impl Indexer {
    pub fn new(len_transports: usize, sources: Sources, sinks: Sinks) -> Self {
        Self {
            len_transports,
            sources,
            sinks,
        }
    }

    pub fn len_sources(&self) -> usize {
        self.sources.len()
    }

    pub fn len_sinks(&self) -> usize {
        self.sinks.len()
    }

    pub fn len_transports(&self) -> usize {
        self.len_transports
    }

    pub fn num_vertices(&self) -> usize {
        self.len_sources() + self.len_sinks() + self.len_transports()
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
