#[derive(Default, Clone)]
pub struct AonWaitDdMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default, Clone)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
