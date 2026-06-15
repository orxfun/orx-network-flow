use crate::cost::Cost;
use crate::flow_units::FlowUnit;
use crate::graphs::{Edge, Graph, VecEdge, Vertex};
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::vars::RoVars;
use crate::networks::{ConnWaitEdge, ConnWaitNw, ConnWaitVertex};
use crate::utils::math_model::{lp_solvers_model_to_lp_file, lp_solvers_model_to_problem};
use crate::{TransportData, Variant};
use alloc::{format, string::ToString};
use good_lp::solvers::lp_solvers::Cplex;
use good_lp::{
    Expression, LpSolver, ProblemVariables, Solution, Solver, SolverModel, Variable,
    VariableDefinition, constraint,
};
use lp_solvers::lp_format::LpProblem;

pub fn add_flow_balance_constraints<'a, V: Variant, S: Solver>(
    nw: &ConnWaitNw<'a, V>,
    ro_vars: &RoVars<'a, V>,
    model: &mut S::Model,
) {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), &nw.g());

    for vertex in g.vertices() {
        let mut out_minus_in = Expression::default();

        for e in vertex.out_edges() {
            // out_minus_in.add_mul(1, vars[e]);
        }
    }

    // for vertex in g.vertices() {
    //     let mut out_minus_in = Expression::default();

    //     for e in vertex.out_edges() {
    //         out_minus_in.add_mul(1, vars[e]);
    //     }

    //     for e in vertex.in_edges() {
    //         out_minus_in.add_mul(-1, vars[e]);
    //     }

    //     let b = match vertex.data() {
    //         ConnWaitVertex::ReadyOri(ro) => {
    //             let commodities = p.sorted_ro_commodities.value_by_key_unc(ro);
    //             let commodities = commodities.iter().map(|&c| p.commodity_by_idx(c));
    //             let demand = commodities.map(|c| c.amount());
    //             FlowUnit::sum(demand).into_f64()
    //         }
    //         ConnWaitVertex::DueDes(dd) => {
    //             let commodities = p.sorted_dd_commodities.value_by_key_unc(dd);
    //             let commodities = commodities.iter().map(|&c| p.commodity_by_idx(c));
    //             let demand = commodities.map(|c| c.amount());
    //             -FlowUnit::sum(demand).into_f64()
    //         }
    //         ConnWaitVertex::Transport(_) => 0.0,
    //     };

    //     let mut constraint = constraint!(out_minus_in == b);

    //     if named {
    //         let name = match vertex.data() {
    //             ConnWaitVertex::ReadyOri(ro) => {
    //                 let ori = p.space_key(ro.space());
    //                 format!("fb_exit__{ori}_{}", ro.time())
    //             }
    //             ConnWaitVertex::DueDes(dd) => {
    //                 let des = p.space_key(dd.space());
    //                 format!("fb_exit__{des}_{}", dd.time())
    //             }
    //             ConnWaitVertex::Transport(t) => {
    //                 let t = p.transport_by_idx(*t);
    //                 format!("fb_tra__{}", t.var_str(p))
    //             }
    //         };
    //         constraint = constraint.set_name(name);
    //     }

    //     model.add_constraint(constraint);
    // }
}
