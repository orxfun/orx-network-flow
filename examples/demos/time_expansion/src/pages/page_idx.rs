#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageIdx {
    #[default]
    Problem,
    Network,
}

impl PageIdx {
    pub const ALL: [Self; 2] = [Self::Problem, Self::Network];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Problem => "Problem",
            Self::Network => "Network",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Problem => "Explore the time-expanded problem setup and inputs.",
            Self::Network => "Inspect the generated network structure and flow behavior.",
        }
    }
}
