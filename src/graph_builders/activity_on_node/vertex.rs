use crate::graph_builders::activity_on_node::sinks::SinkIdx;
use crate::{commodities::Commodity, transports::Transport};

pub enum VertexData {
    Source(Commodity),
    Sink(SinkIdx),
    Transport(Transport),
}
