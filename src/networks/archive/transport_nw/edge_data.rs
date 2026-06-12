use crate::Variant;

#[derive(Clone, Copy)]
pub enum TrDe<V: Variant> {
    Waiting,
    Transport { capacity: V::F },
}
