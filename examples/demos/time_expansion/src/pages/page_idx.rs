use crate::pages::{network::NetworkViewIdx, problem::ProblemViewIdx};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageIdx {
    Problem(ProblemViewIdx),
    Network(NetworkViewIdx),
}

impl Default for PageIdx {
    fn default() -> Self {
        Self::Problem(ProblemViewIdx::Spaces)
    }
}

impl PageIdx {
    pub const ALL: [Self; 2] = [
        Self::Problem(ProblemViewIdx::Spaces),
        Self::Network(NetworkViewIdx::ConnectionSettings),
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Problem(_) => "Problem",
            Self::Network(_) => "Network",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Problem(_) => "Explore the time-expanded problem setup and inputs.",
            Self::Network(_) => "Inspect the generated network structure and flow behavior.",
        }
    }

    pub fn key(self) -> usize {
        match self {
            PageIdx::Problem(_) => 0,
            PageIdx::Network(_) => 1,
        }
    }
}
