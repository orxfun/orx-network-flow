use crate::{commodities::Commodity, transports::Transport};

pub enum VertexData {
    Source(Commodity),
    Sink(Commodity),
    Transport(Transport),
}
