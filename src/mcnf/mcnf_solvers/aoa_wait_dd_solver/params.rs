#[derive(Default, Clone)]
pub struct AoaWaitDdMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default, Clone)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
