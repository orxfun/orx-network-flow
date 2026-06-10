#[derive(derive_new::new)]
pub enum ComOdStDe {
    SourceTransport,
    TransportTransport,
    TransportSink,
    SourceSink,
}
