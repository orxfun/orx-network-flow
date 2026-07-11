#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NetworkViewIdx {
    #[default]
    ConnectionSettings,
}

impl NetworkViewIdx {
    pub const ALL: [Self; 1] = [Self::ConnectionSettings];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ConnectionSettings => "Connection Settings",
        }
    }

    pub const ALL_KEYS: [&str; 1] = [Self::ConnectionSettings.label()];

    pub fn from_label(label: &str) -> Self {
        Self::ALL_KEYS
            .iter()
            .enumerate()
            .find(|(_, x)| **x == label)
            .map(|(i, _)| Self::ALL[i])
            .expect("Unknown problem view label")
    }
}
