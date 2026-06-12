use crate::cost::Cost;
use crate::flow_units::FlowUnit;
use crate::graphs::{EIdx, Edge, Graph, VecEdge, Vertex};
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitNw, ConnWaitVertex};
use crate::{Transport, TransportData, Variant};
use alloc::{format, string::ToString};
use good_lp::solvers::lp_solvers::{Cplex, Model};
use good_lp::variable::UnsolvedProblem;
use good_lp::{
    Constraint, Expression, LpSolver, ProblemVariables, Solution, Solver, SolverModel, Variable,
    VariableDefinition,
};
use std::dbg;

pub fn cplex_solver() -> LpSolver<Cplex> {
    good_lp::LpSolver(Cplex::with_command(
        "/usr/local/cplex/bin/x86-64_linux/cplex".to_string(),
    ))
}

pub fn solve<V>(nw: &ConnWaitNw<'_, V>, named: bool)
where
    V: Variant,
{
    let (p, g) = (nw.p, &nw.g);
    let t_str = |t: &TransportData<V>| t.var_str(p);
    let mut pr_vars = ProblemVariables::new();

    let mut vars = VecEdge::new();

    for e in g.edges() {
        let mut var = VariableDefinition::new().min(0);

        if named {
            let [i, j] = [e.tail(), e.head()].map(|x| g.vertex(x));
            let [tail, head] = [i.data(), j.data()];
            match e.data() {
                ConnWaitEdge::Enter => {
                    let ro = tail.get_ro().expect("ro").0;
                    let ori = p.space_key(ro.space());
                    let t = p.transport_by_idx(head.get_t().expect("t"));
                    var = var.name(format!("enter__{ori}_{}__{}", ro.time(), t_str(t)));
                }
                ConnWaitEdge::Connect => {
                    let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
                    let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
                    var = var.name(format!("con__{}__{}", t_str(t1), t_str(t2)));
                }
                ConnWaitEdge::Wait => {
                    let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
                    let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
                    var = var.name(format!("wait__{}__{}", t_str(t1), t_str(t2)));
                }
                ConnWaitEdge::Exit => {
                    let dd = head.get_dd().expect("dd").0;
                    let des = p.space_key(dd.space());
                    let t = p.transport_by_idx(tail.get_t().expect("t"));
                    var = var.name(format!("exit__{des}_{}__{}", dd.time(), t_str(t)));
                }
                ConnWaitEdge::Bypass(c) => {
                    let com = p.commodity_by_idx(*c);
                    var = var.name(format!("bypass__{}", com.var_str(p)));
                }
            }
        }

        vars.push(pr_vars.add(var));
    }

    let objective = objective(nw, &vars);

    let mut model = pr_vars.minimise(objective).using(cplex_solver());
    flow_balance::<_, LpSolver<Cplex>>(nw, &vars, &mut model, named);

    let solution = model.solve().expect("Failed to solve");

    for x in vars.iter() {
        let b = solution.value(*x);
        if b > 0.0 {
            dbg!(b);
        }
    }
}

fn objective<V>(nw: &ConnWaitNw<'_, V>, vars: &VecEdge<Variable>) -> Expression
where
    V: Variant,
{
    let (p, g) = (nw.p, &nw.g);
    let mut cost = Expression::default();

    let bypass_edges_data = g.edges_slice(nw.bypass_edges_range);
    let bypass_edge_indices = nw.bypass_edges_range.iter();
    for (e, edge) in bypass_edge_indices.zip(bypass_edges_data) {
        let c = edge.data().get_bypass_c().expect("bypass");
        let coef = p.costs.lost_revenue.cost(c);
        cost.add_mul(coef.into_f64(), vars[e]);
    }

    cost
}

fn flow_balance<V, S: Solver>(
    nw: &ConnWaitNw<'_, V>,
    vars: &VecEdge<Variable>,
    model: &mut S::Model,
    named: bool,
) where
    V: Variant,
{
    let (p, g) = (nw.p, &nw.g);

    for vertex in g.vertices() {
        let mut out_minus_in = Expression::default();

        for e in vertex.out_edges() {
            out_minus_in.add_mul(1, vars[e]);
        }

        for e in vertex.in_edges() {
            out_minus_in.add_mul(-1, vars[e]);
        }

        let b = match vertex.data() {
            ConnWaitVertex::ReadyOri(_, commodities) => {
                let commodities = commodities.iter().map(|&c| p.commodity_by_idx(c));
                let demand = commodities.map(|c| c.amount());
                FlowUnit::sum(demand).into_f64()
            }
            ConnWaitVertex::DueDes(_, commodities) => {
                let commodities = commodities.iter().map(|&c| p.commodity_by_idx(c));
                let demand = commodities.map(|c| c.amount());
                -FlowUnit::sum(demand).into_f64()
            }
            ConnWaitVertex::Transport(_) => 0.0,
        };

        let mut constraint = out_minus_in.eq(b);
        if named {
            let name = match vertex.data() {
                ConnWaitVertex::ReadyOri(ro, _) => {
                    let ori = p.space_key(ro.space());
                    format!("fb_enter__{ori}_{}", ro.time())
                }
                ConnWaitVertex::DueDes(dd, _) => {
                    let des = p.space_key(dd.space());
                    format!("fb_enter__{des}_{}", dd.time())
                }
                ConnWaitVertex::Transport(t) => {
                    let t = p.transport_by_idx(*t);
                    format!("fb_tra__{}", t.var_str(p))
                }
            };
            constraint = constraint.set_name(name);
        }

        model.add_constraint(constraint);
    }
}
