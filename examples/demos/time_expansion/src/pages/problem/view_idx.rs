#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProblemViewIdx {
    #[default]
    Spaces,
    Commodities,
    Transports,
}

impl ProblemViewIdx {
    pub const ALL: [Self; 3] = [Self::Spaces, Self::Commodities, Self::Transports];

    pub const ALL_KEYS: [&str; 3] = ["Spaces", "Commodities", "Transports"];
}
