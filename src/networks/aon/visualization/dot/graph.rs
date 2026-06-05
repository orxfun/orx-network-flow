use crate::graph_builders::activity_on_node::visualization::dot::AonDotGraphSettings;
use crate::{AonNetwork, Problem, Variant};

pub struct AonDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    network: &'a AonNetwork,
    settings: AonDotGraphSettings,
}
