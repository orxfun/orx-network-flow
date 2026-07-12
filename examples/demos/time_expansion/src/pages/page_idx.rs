#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageIdx {
    Problem,
    Network,
}

impl Default for PageIdx {
    fn default() -> Self {
        Self::Problem
    }
}

impl PageIdx {
    pub const ALL: [Self; 2] = [Self::Problem, Self::Network];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Problem => "Problem",
            Self::Network => "Network",
        }
    }

    pub const ALL_KEYS: [&str; 2] = [Self::Problem.label(), Self::Network.label()];

    pub const fn description(self) -> &'static str {
        match self {
            Self::Problem => "Explore the time-expanded problem setup and inputs.",
            Self::Network => "Inspect the generated network structure and flow behavior.",
        }
    }

    pub fn from_label(label: &str) -> Self {
        Self::ALL_KEYS
            .iter()
            .enumerate()
            .find(|(_, x)| **x == label)
            .map(|(i, _)| Self::ALL[i])
            .expect("Unknown problem view label")
    }
}
