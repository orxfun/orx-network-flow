use crate::networks::core::{sinks::SinkIdx, sources::SourceIdx};
use crate::transports::Transport;

pub enum AonVertex {
    Transport(Transport),
    Source(SourceIdx),
    Sink(SinkIdx),
}
