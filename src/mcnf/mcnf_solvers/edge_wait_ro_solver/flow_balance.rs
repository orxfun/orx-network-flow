use crate::graphs::core::VertexCore;
use crate::graphs::{Graph, Vertex};
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::vars::RoVars;
use crate::networks::{ConnWaitNw, ConnWaitVertex};
use crate::{FlowUnit, Problem, SpaceTime, Variant};
use alloc::{format, string::String};
use good_lp::{Expression, Solver, SolverModel, constraint};

pub fn add_flow_balance_constraints<'a, V: Variant, S: Solver>(
    nw: &ConnWaitNw<'a, V>,
    ro_vars: &RoVars<'a, V>,
    model: &mut S::Model,
) {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), &nw.g());

    for (ro, vars) in ro_vars.iter() {
        let ro_commodities = p.sorted_ro_commodities.value_by_key_unc(&ro);

        for vertex in g.vertices() {
            let mut out_minus_in = Expression::default();

            for e in vertex.out_edges() {
                out_minus_in.add_mul(1, vars[e]);
            }

            for e in vertex.in_edges() {
                out_minus_in.add_mul(-1, vars[e]);
            }

            let b = match vertex.data() {
                ConnWaitVertex::ReadyOri(vertex_ro) => match *vertex_ro == ro {
                    true => {
                        let commodities = ro_commodities.iter().map(|&c| p.commodity_by_idx(c));
                        let demand = commodities.map(|c| c.amount());
                        FlowUnit::sum(demand).into_f64()
                    }
                    false => 0.0,
                },
                ConnWaitVertex::DueDes(dd) => {
                    let commodities = ro_commodities.iter().map(|&c| p.commodity_by_idx(c));
                    let commodities = commodities.filter(|c| c.destination() == *dd);
                    let demand = commodities.map(|c| c.amount());
                    -FlowUnit::sum(demand).into_f64()
                }
                ConnWaitVertex::Transport(_) => 0.0,
            };

            let constraint = constraint!(out_minus_in == b);

            let constraint = match named {
                true => constraint.set_name(constraint_name(p, ro, vertex)),
                false => constraint,
            };

            model.add_constraint(constraint);
        }
    }
}

fn constraint_name<V: Variant>(
    p: &Problem<V>,
    ro: SpaceTime,
    vertex: &VertexCore<ConnWaitVertex>,
) -> String {
    let o = p.space_key(ro.space());
    let r = ro.time();
    match vertex.data() {
        ConnWaitVertex::ReadyOri(ro) => {
            let ori = p.space_key(ro.space());
            format!("fb_enter__{o}_{r}__{ori}_{}", ro.time())
        }
        ConnWaitVertex::DueDes(dd) => {
            let des = p.space_key(dd.space());
            format!("fb_exit__{o}_{r}__{des}_{}", dd.time())
        }
        ConnWaitVertex::Transport(t) => {
            let t = p.transport_by_idx(*t);
            format!("fb_tra__{o}_{r}__{}", t.var_str(p))
        }
    }
}
