#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProblemViewIdx {
    #[default]
    Spaces,
    Commodities,
    Transports,
}

impl ProblemViewIdx {
    pub const ALL: [Self; 3] = [Self::Spaces, Self::Commodities, Self::Transports];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Spaces => "Spaces",
            Self::Commodities => "Commodities",
            Self::Transports => "Transports",
        }
    }

    pub const ALL_KEYS: [&str; 3] = [
        Self::Spaces.label(),
        Self::Commodities.label(),
        Self::Transports.label(),
    ];

    pub fn from_label(label: &str) -> Self {
        Self::ALL_KEYS
            .iter()
            .enumerate()
            .find(|(_, x)| **x == label)
            .map(|(i, _)| Self::ALL[i])
            .expect("Unknown problem view label")
    }
}
