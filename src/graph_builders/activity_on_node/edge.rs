use crate::commodities::Commodity;

pub enum EdgeData {
    SourceToSourceWait(Commodity, Commodity),
    SinkToSinkWait,
    TransportToTransportWait,
    SourceToSink,
    SourceToTransport,
    TransportToTransport,
    TransportToSink,
}
