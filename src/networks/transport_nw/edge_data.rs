use crate::Variant;

pub enum TransportEd<V: Variant> {
    Waiting,
    Transport { capacity: V::F },
}
