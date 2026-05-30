use crate::commodities::Commodity;

pub enum EdgeData {
    SourceToSourceWait(Commodity, Commodity),
    SinkToSinkWait(Commodity, Commodity),
    TransportToTransportWait,
    SourceToSink,
    SourceToTransport,
    TransportToTransport,
    TransportToSink,
}
