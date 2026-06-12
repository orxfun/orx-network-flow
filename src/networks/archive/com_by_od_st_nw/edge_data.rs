use crate::{Variant, networks::transport_nw::TrDe};

pub enum ComOdStDe<V: Variant> {
    SourceTransport,
    TransportTransport(TrDe<V>),
    TransportSink,
    SourceSink,
}
