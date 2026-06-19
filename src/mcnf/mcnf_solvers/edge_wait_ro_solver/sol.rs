use crate::mcnf::mcnf_solvers::edge_wait_ro_solver::vars::RoVars;
use crate::mcnf::solution::CommodityLoad;
use crate::{FlowUnit, IdxCore, McnfSolution, Variant, VecTransport};
use crate::{commodities::VecCommodity, networks::ConnWaitNw};
use alloc::vec::Vec;
use good_lp::{Solution, Solver, SolverModel, Variable};

pub fn create_solution<V: Variant, S: Solver>(
    nw: &ConnWaitNw<'_, V>,
    ro_vars: &RoVars<'_, V>,
    solution: &<S::Model as SolverModel>::Solution,
) -> McnfSolution<V> {
    let p = nw.p();
    let var_to_flow = |x: &Variable| <V::F as FlowUnit>::from_f64(solution.value(*x));

    let mut transport_loads: VecTransport<Vec<CommodityLoad<V>>> =
        VecTransport::new_filled(p.len_transports(), Default::default);

    let bypass_edge_by_commodity = nw.bypass_edge_by_commodity();

    for (ro, vars) in ro_vars.iter() {
        let commodities = p.sorted_ro_commodities.value_by_key_unc(&ro);

        let mut remaining_by_commodity = Vec::with_capacity(commodities.len());
        let mut total_remaining = 0.0;
        for &c in commodities {
            let amount = p.commodity_by_idx(c).amount();
            let bypass = bypass_edge_by_commodity[c]
                .map(|e| var_to_flow(&vars[e]))
                .unwrap_or_default();
            let remaining = (amount - bypass).into_f64().max(0.0);

            if remaining > 0.0 {
                total_remaining += remaining;
                remaining_by_commodity.push((c, remaining));
            }
        }

        if total_remaining <= 0.0 {
            continue;
        }

        for (t, edges) in nw.transport_edges() {
            let total_load_on_transport =
                edges.iter().map(|&e| solution.value(vars[e])).sum::<f64>();

            if total_load_on_transport <= 0.0 {
                continue;
            }

            let loads = &mut transport_loads[t];
            for (commodity, remaining) in &remaining_by_commodity {
                let ratio = *remaining / total_remaining;
                let load = <V::F as FlowUnit>::from_f64(total_load_on_transport * ratio);
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
