use crate::{commodities::Commodity, transports::Transport};

pub enum VertexAon {
    Source(Commodity),
    Sink(Commodity),
    Transport(Transport),
}
