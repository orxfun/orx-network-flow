use crate::{commodities::Commodity, transports::Transport};

pub enum EdgeData {
    SourceToSink(Commodity),
    SourceToTransport(Commodity, Transport),
    TransportToTransport(Transport, Transport),
    TransportToSink(Transport, Commodity),
}
