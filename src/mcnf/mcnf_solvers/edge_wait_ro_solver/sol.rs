use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::vars::RoVars;
use crate::{FlowUnit, McnfSolution, Variant, VecTransport};
use crate::{commodities::VecCommodity, networks::ConnWaitNw};
use good_lp::{Solution, Solver, SolverModel, Variable};

pub fn create_solution<V: Variant, S: Solver>(
    nw: &ConnWaitNw<'_, V>,
    ro_vars: &RoVars<'_, V>,
    solution: &<S::Model as SolverModel>::Solution,
) -> McnfSolution<V> {
    let p = nw.p();
    let var_to_flow = |x: &Variable| <V::F as FlowUnit>::from_f64(solution.value(*x));

    let mut transport_loads = VecTransport::new_filled(p.len_transports(), Default::default);
    for (t, edges) in nw.transport_edges() {
        for (ro, vars) in ro_vars.iter() {
            //
        }
        let x = edges.iter().next().unwrap();
    }

    let mut commodity_paths = VecCommodity::new_filled(p.len_commodities(), Default::default);

    // let var_to_flow = |x: &Variable| FlowUnit::from_f64(solution.value(*x));

    McnfSolution::new(commodity_paths, transport_loads)
}
