use crate::cost::Cost;
use crate::flow_units::FlowUnit;
use crate::graphs::{Edge, Graph, VecEdge, Vertex};
use crate::networks::{ConnWaitEdge, ConnWaitNw, ConnWaitVertex};
use crate::{Problem, Variant};
use good_lp::{
    Expression, LpSolver, ProblemVariables, Solution, Solver, SolverModel, Variable,
    VariableDefinition, constraint,
};
use lp_solvers::lp_format::LpProblem;

pub fn define_vars<V: Variant>(nw: &ConnWaitNw<'_, V>) {
    //
}

fn define_vars_ro<V: Variant>(nw: &ConnWaitNw<'_, V>, ro_idx: usize) {
    let (p, g) = (nw.p(), &nw.g());

    for e in g.edges() {
        let mut var = VariableDefinition::new().min(0);

        if let ConnWaitEdge::Bypass(c) = e.data() {
            let amount = p.commodity_by_idx(*c).amount().into_f64();
            var = var.max(amount);
        }
    }
}
