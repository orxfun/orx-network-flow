use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::AoaWaitDdMcnfParams;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::disaggregate_greedy::disaggregate_dd_greedy;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::params::DisaggregationStrategy;
use crate::mcnf::mcnf_solvers::aoa_wait_dd_solver::vars::DdVars;
use crate::mcnf::solution::CommodityLoad;
use crate::{FlowUnit, McnfSolution, Variant, VecTransport};
use crate::{commodities::VecCommodity, networks::AoaWaitNw};
use alloc::vec::Vec;
use good_lp::{Solution, Solver, SolverModel, Variable};

pub fn create_solution<V: Variant, S: Solver>(
    nw: &AoaWaitNw<'_, V>,
    params: &AoaWaitDdMcnfParams,
    dd_vars: &DdVars<'_, V>,
    solution: &<S::Model as SolverModel>::Solution,
) -> McnfSolution<V> {
    let p = nw.p();
    let var_to_flow = |x: &Variable| FlowUnit::from_f64(solution.value(*x));

    let mut commodity_paths = VecCommodity::new_filled(p.len_commodities(), Default::default);

    let mut transport_loads: VecTransport<Vec<CommodityLoad<V>>> =
        VecTransport::new_filled(p.len_transports(), Default::default);

    for (dd, vars) in dd_vars.iter() {
        let edge_flow = |e| var_to_flow(&vars[e]);

        match params.disaggregation {
            DisaggregationStrategy::Greedy => disaggregate_dd_greedy(
                nw,
                dd,
                edge_flow,
                &mut transport_loads,
                &mut commodity_paths,
            ),
        }
    }

    McnfSolution::new(commodity_paths, transport_loads)
}
