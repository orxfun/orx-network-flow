use crate::{commodities::Commodity, transports::Transport};

pub enum EdgeData {
    SourceToSourceWait(Commodity, Commodity),
    SinkToSinkWait(Commodity, Commodity),
    TransportToTransportWait(Transport, Transport),
    SourceToSink(Commodity),
    SourceToTransport(Commodity, Transport),
    TransportToTransport(Transport, Transport),
    TransportToSink(Transport, Commodity),
}
