#[derive(Default)]
pub struct AoaWaitRoMcnfParams {
    pub disaggregation: DisaggregationStrategy,
}

#[derive(Default)]
pub enum DisaggregationStrategy {
    #[default]
    Greedy,
}
