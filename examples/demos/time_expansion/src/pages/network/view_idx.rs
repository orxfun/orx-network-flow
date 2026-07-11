#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NetworkViewIdx {
    #[default]
    ConnectionSettings,
}

impl NetworkViewIdx {
    pub const ALL: [Self; 1] = [Self::ConnectionSettings];
}
