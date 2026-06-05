use crate::graph_builders::activity_on_node::visualization::dot::AonDotGraphSettings;
use crate::{AonNetwork, Problem, Variant};

pub struct AonDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    network: &'a AonNetwork,
    settings: AonDotGraphSettings,
}

impl<'a, V: Variant> AonDotGraph<'a, V> {
    pub fn new(problem: &'a Problem<V>, network: &'a AonNetwork) -> Self {
        Self::with_settings(problem, network, Default::default())
    }

    pub fn with_settings(
        problem: &'a Problem<V>,
        network: &'a AonNetwork,
        settings: AonDotGraphSettings,
    ) -> Self {
        Self {
            problem,
            network,
            settings,
        }
    }
}
