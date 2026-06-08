use crate::transports::Transport;

#[derive(derive_new::new)]
pub struct TrDv {
    t: Transport,
}
