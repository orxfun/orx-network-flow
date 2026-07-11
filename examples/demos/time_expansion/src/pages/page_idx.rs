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

    pub const fn description(self) -> &'static str {
        match self {
            Self::Problem => "Explore the time-expanded problem setup and inputs.",
            Self::Network => "Inspect the generated network structure and flow behavior.",
        }
    }

    pub fn key(self) -> usize {
        match self {
            PageIdx::Problem => 0,
            PageIdx::Network => 1,
        }
    }
}
