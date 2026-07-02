#[derive(Default)]
pub struct AoaWaitDdMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
