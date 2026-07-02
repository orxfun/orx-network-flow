use crate::graphs::core::VertexCore;
use crate::graphs::{Graph, Vertex};
use crate::mcnf::mcnf_solvers::aon_wait_dd_solver::vars::DdVars;
use crate::networks::{AonWaitNw, AonWaitVertex};
use crate::{FlowUnit, Problem, SpaceTime, Variant};
use alloc::{format, string::String};
use good_lp::{Expression, Solver, SolverModel, constraint};

pub fn add_flow_balance_constraints<'a, V: Variant, S: Solver>(
    nw: &AonWaitNw<'a, V>,
    dd_vars: &DdVars<'a, V>,
    model: &mut S::Model,
) {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), &nw.g());

    for (dd, vars) in dd_vars.iter() {
        let dd_commodities = p.sorted_dd_commodities.value_by_key_unc(&dd);
        let total_demand = FlowUnit::sum(
            dd_commodities
                .iter()
                .map(|&c| p.commodity_by_idx(c).amount()),
        );

        for vertex in g.vertices() {
            let mut out_minus_in = Expression::default();

            for e in vertex.out_edges() {
                out_minus_in.add_mul(1, vars[e]);
            }

            for e in vertex.in_edges() {
                out_minus_in.add_mul(-1, vars[e]);
            }

            let b = match vertex.data() {
                AonWaitVertex::ReadyOri(vertex_ro) => {
                    let commodities = dd_commodities.iter().map(|&c| p.commodity_by_idx(c));
                    let commodities = commodities.filter(|c| c.origin() == *vertex_ro);
                    let demand = commodities.map(|c| c.amount());
                    FlowUnit::sum(demand).into_f64()
                }
                AonWaitVertex::DueDes(vertex_dd) => {
                    if *vertex_dd == dd {
                        -total_demand.into_f64()
                    } else {
                        0.0
                    }
                }
                AonWaitVertex::Transport(_) => 0.0,
            };

            let constraint = constraint!(out_minus_in == b);

            let constraint = match named {
                true => constraint.set_name(constraint_name(p, dd, vertex)),
                false => constraint,
            };

            model.add_constraint(constraint);
        }
    }
}

fn constraint_name<V: Variant>(
    p: &Problem<V>,
    dd: SpaceTime,
    vertex: &VertexCore<AonWaitVertex>,
) -> String {
    let d = p.space_key(dd.space());
    let due = dd.time();
    match vertex.data() {
        AonWaitVertex::ReadyOri(ro) => {
            let ori = p.space_key(ro.space());
            format!("fb_enter__{d}_{due}__{ori}_{}", ro.time())
        }
        AonWaitVertex::DueDes(dd) => {
            let des = p.space_key(dd.space());
            format!("fb_exit__{d}_{due}__{des}_{}", dd.time())
        }
        AonWaitVertex::Transport(t) => {
            let t = p.transport_by_idx(*t);
            format!("fb_tra__{d}_{due}__{}", t.var_str(p))
        }
    }
}
