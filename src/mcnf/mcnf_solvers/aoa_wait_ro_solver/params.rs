#[derive(Default, Clone)]
pub struct AoaWaitRoMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default, Clone)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
