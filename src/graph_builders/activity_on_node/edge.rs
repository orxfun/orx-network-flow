use crate::{commodities::Commodity, transports::Transport};

pub enum EdgeData {
    SinkToSinkWait(Commodity, Commodity),
    SourceToSink(Commodity),
    SourceToTransport(Commodity, Transport),
    TransportToTransport(Transport, Transport),
    TransportToSink(Transport, Commodity),
}
