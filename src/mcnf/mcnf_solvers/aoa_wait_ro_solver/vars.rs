use crate::flow_units::FlowUnit;
use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::{Edge, Graph, VecEdge, Vertex};
use crate::networks::{AoaWaitEdge, AoaWaitNw, AoaWaitVertex};
use crate::{Problem, SpaceTime, TransportData, Variant};
use alloc::{format, string::String, vec::Vec};
use good_lp::{ProblemVariables, Variable, VariableDefinition};

pub struct RoVars<'a, V: Variant> {
    p: &'a Problem<V>,
    /// Variables per unique ready-origin space-time, parallel to
    /// `sorted_ro_commodities` of the corresponding problem `p`.
    vars: Vec<VecEdge<Variable>>,
}

impl<V: Variant> RoVars<'_, V> {
    pub fn ro(&self) -> impl Iterator<Item = SpaceTime> {
        self.p.sorted_ro_commodities.keys().copied()
    }

    pub fn vars_of(&self, ro: SpaceTime) -> &VecEdge<Variable> {
        let ro_idx = self
            .p
            .sorted_ro_commodities
            .key_to_idx(&ro)
            .expect("exists");
        &self.vars[ro_idx]
    }

    pub fn iter(&self) -> impl Iterator<Item = (SpaceTime, &VecEdge<Variable>)> {
        let ro_idx = self.p.sorted_ro_commodities.keys_indices();
        ro_idx.map(|(ro, idx)| (*ro, &self.vars[idx]))
    }
}

pub fn define_vars<'a, V: Variant>(
    nw: &'a AoaWaitNw<'_, V>,
) -> (ProblemVariables, RoVars<'a, V>) {
    let mut pr_vars = ProblemVariables::new();
    let mut ro_vars = Vec::new();

    let dummy = VariableDefinition::new().min(0).max(0);
    let dummy = pr_vars.add(dummy);

    for ro in nw.p().sorted_ro_commodities.keys() {
        let vars = define_vars_ro(*ro, nw, &mut pr_vars, dummy);
        ro_vars.push(vars);
    }

    let ro_vars = RoVars {
        p: nw.p(),
        vars: ro_vars,
    };

    (pr_vars, ro_vars)
}

fn define_vars_ro<V: Variant>(
    ro: SpaceTime,
    nw: &AoaWaitNw<'_, V>,
    pr_vars: &mut ProblemVariables,
    dummy: Variable,
) -> VecEdge<Variable> {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), nw.g());
    let mut vars = VecEdge::new();

    for e in g.edges() {
        let mut var = VariableDefinition::new().min(0);

        let include_in_ro = match e.data() {
            AoaWaitEdge::Bypass(c) => {
                let amount = p.commodity_by_idx(*c).amount().into_f64();
                var = var.max(amount);
                // only include bypass if this commodity belongs to this ro
                p.commodity_by_idx(*c).origin() == ro
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
    g: &GraphCore<AoaWaitVertex, AoaWaitEdge>,
    ro: SpaceTime,
    e: &EdgeCore<AoaWaitEdge>,
) -> String {
    let ro_str = format!("{}_{}", p.space_key(ro.space()), ro.time());
    let t_str = |t: &TransportData<V>| t.var_str(p);

    match e.data() {
        AoaWaitEdge::Transport(t) => {
            let td = p.transport_by_idx(*t);
            format!("{ro_str}__arc__{}", t_str(td))
        }
        AoaWaitEdge::Wait => {
            let tail_st = g.vertex(e.tail()).data().0;
            let head_st = g.vertex(e.head()).data().0;
            let tail_s = p.space_key(tail_st.space());
            format!(
                "{ro_str}__wait__{tail_s}_{}__{}",
                tail_st.time(),
                head_st.time()
            )
        }
        AoaWaitEdge::Bypass(c) => {
            let com = p.commodity_by_idx(*c);
            format!("{ro_str}__bypass__{}", com.var_str(p))
        }
    }
}
