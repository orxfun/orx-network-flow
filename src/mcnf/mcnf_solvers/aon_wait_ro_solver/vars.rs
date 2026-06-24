use crate::flow_units::FlowUnit;
use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::{Edge, Graph, VecEdge, Vertex};
use crate::networks::{AonWaitEdge, AonWaitNw, AonWaitVertex};
use crate::{Problem, SpaceTime, TransportData, Variant};
use alloc::{format, string::String, vec::Vec};
use good_lp::{ProblemVariables, Variable, VariableDefinition};

pub struct RoVars<'a, V: Variant> {
    p: &'a Problem<V>,
    /// Variables per unique ready-origin time-space, which is parallel to
    /// `sorted_ro_commodities` of the corresponding problem `p`.
    vars: Vec<VecEdge<Variable>>,
}

impl<V: Variant> RoVars<'_, V> {
    pub fn ro(&self) -> impl Iterator<Item = SpaceTime> {
        self.p.sorted_ro_commodities.keys().copied()
    }

    pub fn vars_of(&self, ro: SpaceTime) -> &VecEdge<Variable> {
        let p = self.p;
        let ro_idx = p.sorted_ro_commodities.key_to_idx(&ro).expect("exists");
        &self.vars[ro_idx]
    }

    pub fn iter(&self) -> impl Iterator<Item = (SpaceTime, &VecEdge<Variable>)> {
        let ro_idx = self.p.sorted_ro_commodities.keys_indices();
        ro_idx.map(|(ro, idx)| (*ro, &self.vars[idx]))
    }
}

pub fn define_vars<'a, V: Variant>(nw: &'a AonWaitNw<'_, V>) -> (ProblemVariables, RoVars<'a, V>) {
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
    nw: &AonWaitNw<'_, V>,
    pr_vars: &mut ProblemVariables,
    dummy: Variable,
) -> VecEdge<Variable> {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), &nw.g());
    let mut vars = VecEdge::new();

    for e in g.edges() {
        let mut var = VariableDefinition::new().min(0);

        let include_in_ro = match e.data() {
            AonWaitEdge::Bypass(c) => {
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
    g: &GraphCore<AonWaitVertex, AonWaitEdge>,
    ro: SpaceTime,
    e: &EdgeCore<AonWaitEdge>,
) -> String {
    let t_str = |t: &TransportData<V>| t.var_str(p);
    let ro_str = format!("{}_{}", p.space_key(ro.space()), ro.time());

    let [i, j] = [e.tail(), e.head()].map(|x| g.vertex(x));
    let [tail, head] = [i.data(), j.data()];
    match e.data() {
        AonWaitEdge::Enter => {
            let ro = tail.get_ro().expect("ro");
            let ori = p.space_key(ro.space());
            let t = p.transport_by_idx(head.get_t().expect("t"));
            format!("{ro_str}__enter__{ori}_{}__{}", ro.time(), t_str(t))
        }
        AonWaitEdge::Connect => {
            let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
            let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
            format!("{ro_str}__con__{}__{}", t_str(t1), t_str(t2))
        }
        AonWaitEdge::Wait => {
            let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
            let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
            format!("{ro_str}__wait__{}__{}", t_str(t1), t_str(t2))
        }
        AonWaitEdge::Exit => {
            let dd = head.get_dd().expect("dd");
            let des = p.space_key(dd.space());
            let t = p.transport_by_idx(tail.get_t().expect("t"));
            format!("{ro_str}__exit__{}__{des}_{}", t_str(t), dd.time())
        }
        AonWaitEdge::Bypass(c) => {
            let com = p.commodity_by_idx(*c);
            format!("{ro_str}__bypass__{}", com.var_str(p))
        }
    }
}
