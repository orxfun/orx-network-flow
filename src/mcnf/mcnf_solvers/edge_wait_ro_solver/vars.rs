use crate::cost::Cost;
use crate::flow_units::FlowUnit;
use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::{Edge, Graph, VecEdge, Vertex};
use crate::networks::{ConnWaitEdge, ConnWaitNw, ConnWaitVertex};
use crate::{Problem, SpaceTime, TransportData, Variant, problem};
use alloc::{format, string::String};
use good_lp::{
    Expression, LpSolver, ProblemVariables, Solution, Solver, SolverModel, Variable,
    VariableDefinition, constraint,
};
use lp_solvers::lp_format::LpProblem;

pub fn define_vars<V: Variant>(nw: &ConnWaitNw<'_, V>) {
    let mut pr_vars = ProblemVariables::new();
}

fn define_vars_ro<V: Variant>(
    ro: SpaceTime,
    nw: &ConnWaitNw<'_, V>,
    pr_vars: &mut ProblemVariables,
    dummy: Variable,
) -> VecEdge<Variable> {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), &nw.g());
    let mut vars = VecEdge::new();

    for e in g.edges() {
        let mut var = VariableDefinition::new().min(0);

        let include_in_ro = match e.data() {
            ConnWaitEdge::Bypass(c) => {
                let amount = p.commodity_by_idx(*c).amount().into_f64();
                var = var.max(amount);

                let tail = g.vertex(e.tail()).data().get_ro().expect("ro");
                tail == ro
            }
            _ => true,
        };

        match include_in_ro {
            true => {
                if named {
                    var = var.name(var_name(p, g, ro, e));
                }
                vars.push(pr_vars.add(var))
            }
            false => vars.push(dummy),
        }
    }

    vars
}

fn var_name<V: Variant>(
    p: &Problem<V>,
    g: &GraphCore<ConnWaitVertex, ConnWaitEdge>,
    ro: SpaceTime,
    e: &EdgeCore<ConnWaitEdge>,
) -> String {
    let t_str = |t: &TransportData<V>| t.var_str(p);
    let ro_str = format!("{}_{}", p.space_key(ro.space()), ro.time());

    let [i, j] = [e.tail(), e.head()].map(|x| g.vertex(x));
    let [tail, head] = [i.data(), j.data()];
    match e.data() {
        ConnWaitEdge::Enter => {
            let ro = tail.get_ro().expect("ro");
            let ori = p.space_key(ro.space());
            let t = p.transport_by_idx(head.get_t().expect("t"));
            format!("{ro_str}__enter__{ori}_{}__{}", ro.time(), t_str(t))
        }
        ConnWaitEdge::Connect => {
            let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
            let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
            format!("{ro_str}__con__{}__{}", t_str(t1), t_str(t2))
        }
        ConnWaitEdge::Wait => {
            let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
            let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
            format!("{ro_str}__wait__{}__{}", t_str(t1), t_str(t2))
        }
        ConnWaitEdge::Exit => {
            let dd = head.get_dd().expect("dd");
            let des = p.space_key(dd.space());
            let t = p.transport_by_idx(tail.get_t().expect("t"));
            format!("{ro_str}__exit__{}__{des}_{}", t_str(t), dd.time())
        }
        ConnWaitEdge::Bypass(c) => {
            let com = p.commodity_by_idx(*c);
            format!("{ro_str}__bypass__{}", com.var_str(p))
        }
    }
}
