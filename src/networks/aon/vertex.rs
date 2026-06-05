use crate::commodities::Commodity;
use crate::networks::aon::{sinks::SinkIdx, sources::SourceIdx};
use crate::transports::Transport;

pub enum AonVertex {
    Transport(Transport),
    Source(SourceIdx),
    Sink(SinkIdx),
    Teleport(Commodity),
}
