use crate::GraphStats;

pub struct McnfStats {
    pub graph_stats: GraphStats,
    pub num_variables: usize,
    pub num_constraints: usize,
}
