use crate::{Variant, commodities::Commodity, transports::Transport};
use core::iter::Map;

pub struct TransportationCost<V: Variant> {
    global: V::C,
    by_commodity: Map<Commodity, V::C>,
    by_transport: Map<Transport, V::C>,
}
