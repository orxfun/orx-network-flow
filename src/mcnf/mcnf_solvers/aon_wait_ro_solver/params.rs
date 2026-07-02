#[derive(Default, Clone)]
pub struct AonWaitRoMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default, Clone)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
