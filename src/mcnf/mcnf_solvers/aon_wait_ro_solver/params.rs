#[derive(Default)]
pub struct AonWaitRoMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
