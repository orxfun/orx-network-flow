use crate::graphs::EIdx;
use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::vars::RoVars;
use crate::mcnf::solution::CommodityLoad;
use crate::{FlowUnit, McnfSolution, Variant, VecTransport};
use crate::{commodities::VecCommodity, networks::ConnWaitNw};
use alloc::vec::Vec;
use good_lp::{Solution, Solver, SolverModel, Variable};

pub fn create_solution<V: Variant, S: Solver>(
    nw: &ConnWaitNw<'_, V>,
    ro_vars: &RoVars<'_, V>,
    solution: &<S::Model as SolverModel>::Solution,
) -> McnfSolution<V> {
    let p = nw.p();
    let var_to_flow = |x: &Variable| FlowUnit::from_f64(solution.value(*x));

    let mut transport_loads: VecTransport<Vec<CommodityLoad<V>>> =
        VecTransport::new_filled(p.len_transports(), Default::default);

    let bypass_edge_by_commodity = nw.bypass_edge_by_commodity();

    for (ro, vars) in ro_vars.iter() {
        let edge_flow = |e: EIdx| var_to_flow(&vars[e]);

        let commodities = p.sorted_ro_commodities.value_by_key_unc(&ro);

        let mut remaining_by_commodity = Vec::with_capacity(commodities.len());
        let mut total_remaining = <V::F as FlowUnit>::zero();
        for &c in commodities {
            let amount = p.commodity_by_idx(c).amount();
            let bypass = bypass_edge_by_commodity[c]
                .map(edge_flow)
                .unwrap_or_default();
            let remaining = amount - bypass;

            if remaining.is_pos() {
                total_remaining += remaining;
                remaining_by_commodity.push((c, remaining));
            }
        }

        if total_remaining.is_nonpos() {
            continue;
        }

        for (t, edges) in nw.transport_edges() {
            let total_load_on_transport = FlowUnit::sum(edges.iter().copied().map(edge_flow));

            if total_load_on_transport.is_nonpos() {
                continue;
            }

            let loads = &mut transport_loads[t];
            for (commodity, remaining) in &remaining_by_commodity {
                let ratio: V::F = *remaining / total_remaining;
                let load = total_load_on_transport * ratio;
                loads.push(CommodityLoad {
                    commodity: *commodity,
                    load,
                });
            }
        }
    }

    let commodity_paths = VecCommodity::new_filled(p.len_commodities(), Default::default);

    McnfSolution::new(commodity_paths, transport_loads)
}
