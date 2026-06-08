use crate::graphs::core::GraphCore;
use crate::networks::transport_nw::nw::TrNw;
use crate::{Problem, Variant};

pub fn create_tr_nw<V: Variant>(p: &Problem<V>) -> TrNw<'_, V> {
    let mut builder = GraphCore::builder();

    let graph = builder.finish();
    TrNw::new(p, graph)
}
