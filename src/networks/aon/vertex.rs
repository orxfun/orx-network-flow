use crate::{networks::aon::sinks::SinkIdx, transports::Transport};

pub enum AonVertex {
    Transport(Transport),
    Source(usize),
    Sink(SinkIdx),
}
