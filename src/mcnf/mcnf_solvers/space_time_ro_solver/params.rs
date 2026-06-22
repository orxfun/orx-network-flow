#[derive(Default)]
pub struct SpaceTimeRoMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
