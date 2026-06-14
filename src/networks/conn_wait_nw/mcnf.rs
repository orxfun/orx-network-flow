use crate::cost::Cost;
use crate::flow_units::FlowUnit;
use crate::graphs::{Edge, Graph, VecEdge, Vertex};
use crate::networks::conn_wait_nw::output::Output;
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitNw, ConnWaitVertex};
use crate::utils::math_model::{
    FlowsByEdges, lp_solvers_model_to_lp_file, lp_solvers_model_to_problem,
};
use crate::{TransportData, Variant};
use alloc::{format, string::ToString};
use good_lp::solvers::lp_solvers::Cplex;
use good_lp::{
    Expression, LpSolver, ProblemVariables, Solution, Solver, SolverModel, Variable,
    VariableDefinition, constraint,
};
use lp_solvers::lp_format::LpProblem;
use std::println;

pub fn cplex_solver() -> LpSolver<Cplex> {
    good_lp::LpSolver(Cplex::with_command(
        "/usr/local/cplex/bin/x86-64_linux/cplex".to_string(),
    ))
}

pub fn solve<V>(nw: &ConnWaitNw<'_, V>, named: bool) -> Output<V>
where
    V: Variant,
{
    let (p, g) = (nw.p, &nw.g);
    let t_str = |t: &TransportData<V>| t.var_str(p);
    let mut pr_vars = ProblemVariables::new();

    let mut vars = VecEdge::new();
    let mut var_names = VecEdge::new();

    for e in g.edges() {
        let mut var = VariableDefinition::new().min(0);

        if let ConnWaitEdge::Bypass(c) = e.data() {
            let amount = p.commodity_by_idx(*c).amount().into_f64();
            var = var.max(amount);
        }

        if named {
            let [i, j] = [e.tail(), e.head()].map(|x| g.vertex(x));
            let [tail, head] = [i.data(), j.data()];
            let name = match e.data() {
                ConnWaitEdge::Enter => {
                    let ro = tail.get_ro().expect("ro").0;
                    let ori = p.space_key(ro.space());
                    let t = p.transport_by_idx(head.get_t().expect("t"));
                    format!("enter__{ori}_{}__{}", ro.time(), t_str(t))
                }
                ConnWaitEdge::Connect => {
                    let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
                    let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
                    format!("con__{}__{}", t_str(t1), t_str(t2))
                }
                ConnWaitEdge::Wait => {
                    let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
                    let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
                    format!("wait__{}__{}", t_str(t1), t_str(t2))
                }
                ConnWaitEdge::Exit => {
                    let dd = head.get_dd().expect("dd").0;
                    let des = p.space_key(dd.space());
                    let t = p.transport_by_idx(tail.get_t().expect("t"));
                    format!("exit__{}__{des}_{}", t_str(t), dd.time())
                }
                ConnWaitEdge::Bypass(c) => {
                    let com = p.commodity_by_idx(*c);
                    format!("bypass__{}", com.var_str(p))
                }
            };

            var = var.name(&name);
            var_names.push(name);
        }

        vars.push(pr_vars.add(var));
    }

    let objective = objective(nw, &vars);

    let mut model = pr_vars.minimise(objective).using(cplex_solver());
    flow_balance::<_, LpSolver<Cplex>>(nw, &vars, &mut model, named);
    capacity::<_, LpSolver<Cplex>>(nw, &vars, &mut model, named);

    let p = unsafe { lp_solvers_model_to_problem(&model) };
    println!("{}", p.display_lp());
    unsafe { lp_solvers_model_to_lp_file(&model, "target/model.lp") }.unwrap();

    let solution = model.solve().expect("Failed to solve");

    let name_width = var_names.iter().map(|name| name.len()).max().unwrap_or(0);
    for (e, &var) in vars.enumerated_iter() {
        let value = solution.value(var);
        if value > 1e-5 {
            let name = &var_names[e];
            println!("{name:<name_width$}  {value}");
        }
    }

    let var_to_flow = |x: &Variable| FlowUnit::from_f64(solution.value(*x));
    let edge_flows = vars.iter().map(var_to_flow).collect();
    Output::create(nw, edge_flows)
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

fn flow_balance<V: Variant, S: Solver>(
    nw: &ConnWaitNw<'_, V>,
    vars: &VecEdge<Variable>,
    model: &mut S::Model,
    named: bool,
) {
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

        let mut constraint = constraint!(out_minus_in == b);

        if named {
            let name = match vertex.data() {
                ConnWaitVertex::ReadyOri(ro, _) => {
                    let ori = p.space_key(ro.space());
                    format!("fb_exit__{ori}_{}", ro.time())
                }
                ConnWaitVertex::DueDes(dd, _) => {
                    let des = p.space_key(dd.space());
                    format!("fb_exit__{des}_{}", dd.time())
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

fn capacity<V: Variant, S: Solver>(
    nw: &ConnWaitNw<'_, V>,
    vars: &VecEdge<Variable>,
    model: &mut S::Model,
    named: bool,
) {
    for (t, edges) in nw.transport_edges.enumerated_iter() {
        if edges.is_empty() {
            continue;
        }

        let capacity = nw.p.transport_by_idx(t).capacity().into_f64();

        let mut total_flow = Expression::default();
        for &e in edges {
            total_flow.add_mul(1, vars[e]);
        }

        let mut constraint = constraint!(total_flow <= capacity);

        if named {
            let name = format!("cap_{t}");
            constraint = constraint.set_name(name);
        }

        model.add_constraint(constraint);
    }
}
