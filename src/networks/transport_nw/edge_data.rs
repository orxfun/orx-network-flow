use crate::Variant;

pub enum TrDe<V: Variant> {
    Waiting,
    Transport { capacity: V::F },
}
