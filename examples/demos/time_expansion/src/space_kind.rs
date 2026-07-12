#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpaceKind {
    #[default]
    Basic,
    Euclidean,
    Geographic,
}

impl SpaceKind {
    pub const ALL: [Self; 3] = [Self::Basic, Self::Euclidean, Self::Geographic];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Euclidean => "Euclidean",
            Self::Geographic => "Geographic",
        }
    }

    pub const ALL_KEYS: [&str; 3] = [
        Self::Basic.label(),
        Self::Euclidean.label(),
        Self::Geographic.label(),
    ];

    pub fn from_label(label: &str) -> Self {
        Self::ALL_KEYS
            .iter()
            .enumerate()
            .find(|(_, x)| **x == label)
            .map(|(i, _)| Self::ALL[i])
            .expect("Unknown space kind")
    }
}
