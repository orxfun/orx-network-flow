use crate::{Variant, space_time::SpaceTimeOd, transports::Transport};

pub enum ComOdStDv<V: Variant> {
    Transport(Transport),
    OriSt(SpaceTimeOd, V::F),
    DesSt(SpaceTimeOd, V::F),
}
