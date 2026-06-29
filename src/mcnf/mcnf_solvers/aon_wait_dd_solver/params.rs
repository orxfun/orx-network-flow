#[derive(Default)]
pub struct AonWaitDdMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
