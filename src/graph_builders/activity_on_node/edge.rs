use crate::{commodities::Commodity, transports::Transport};

pub enum EdgeData {
    SourceToSourceWait(Commodity, Commodity),
    SinkToSinkWait(Commodity, Commodity),
    TransportToTransportWait(Transport, Transport),
    SourceToSink,
    SourceToTransport(Commodity, Transport),
    TransportToTransport,
    TransportToSink(Transport, Commodity),
}
