use crate::flow_units::FlowUnit;
use crate::graphs::core::{EdgeCore, GraphCore};
use crate::graphs::{Edge, Graph, VecEdge, Vertex};
use crate::networks::{AonWaitEdge, AonWaitNw, AonWaitVertex};
use crate::{Problem, SpaceTime, TransportData, Variant};
use alloc::{format, string::String, vec::Vec};
use good_lp::{ProblemVariables, Variable, VariableDefinition};

pub struct DdVars<'a, V: Variant> {
    p: &'a Problem<V>,
    /// Variables per unique due-destination time-space, which is parallel to
    /// `sorted_dd_commodities` of the corresponding problem `p`.
    vars: Vec<VecEdge<Variable>>,
}

impl<V: Variant> DdVars<'_, V> {
    pub fn dd(&self) -> impl Iterator<Item = SpaceTime> {
        self.p.sorted_dd_commodities.keys().copied()
    }

    pub fn vars_of(&self, dd: SpaceTime) -> &VecEdge<Variable> {
        let p = self.p;
        let dd_idx = p.sorted_dd_commodities.key_to_idx(&dd).expect("exists");
        &self.vars[dd_idx]
    }

    pub fn iter(&self) -> impl Iterator<Item = (SpaceTime, &VecEdge<Variable>)> {
        let dd_idx = self.p.sorted_dd_commodities.keys_indices();
        dd_idx.map(|(dd, idx)| (*dd, &self.vars[idx]))
    }
}

pub fn define_vars<'a, V: Variant>(nw: &'a AonWaitNw<'_, V>) -> (ProblemVariables, DdVars<'a, V>) {
    let mut pr_vars = ProblemVariables::new();
    let mut dd_vars = Vec::new();

    let dummy = VariableDefinition::new().min(0).max(0);
    let dummy = pr_vars.add(dummy);

    for dd in nw.p().sorted_dd_commodities.keys() {
        let vars = define_vars_dd(*dd, nw, &mut pr_vars, dummy);
        dd_vars.push(vars);
    }

    let dd_vars = DdVars {
        p: nw.p(),
        vars: dd_vars,
    };

    (pr_vars, dd_vars)
}

fn define_vars_dd<V: Variant>(
    dd: SpaceTime,
    nw: &AonWaitNw<'_, V>,
    pr_vars: &mut ProblemVariables,
    dummy: Variable,
) -> VecEdge<Variable> {
    let named = cfg!(debug_assertions);
    let (p, g) = (nw.p(), &nw.g());
    let mut vars = VecEdge::new();

    for e in g.edges() {
        let mut var = VariableDefinition::new().min(0);

        let include_in_dd = match e.data() {
            AonWaitEdge::Bypass(c) => {
                let amount = p.commodity_by_idx(*c).amount().into_f64();
                var = var.max(amount);

                p.commodity_by_idx(*c).destination() == dd
            }
            _ => true,
        };

        match include_in_dd {
            true => {
                if named {
                    var = var.name(var_name(p, g, dd, e));
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
    dd: SpaceTime,
    e: &EdgeCore<AonWaitEdge>,
) -> String {
    let t_str = |t: &TransportData<V>| t.var_str(p);
    let dd_str = format!("{}_{}", p.space_key(dd.space()), dd.time());

    let [i, j] = [e.tail(), e.head()].map(|x| g.vertex(x));
    let [tail, head] = [i.data(), j.data()];
    match e.data() {
        AonWaitEdge::Enter => {
            let ro = tail.get_ro().expect("ro");
            let ori = p.space_key(ro.space());
            let t = p.transport_by_idx(head.get_t().expect("t"));
            format!("{dd_str}__enter__{ori}_{}__{}", ro.time(), t_str(t))
        }
        AonWaitEdge::Connect => {
            let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
            let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
            format!("{dd_str}__con__{}__{}", t_str(t1), t_str(t2))
        }
        AonWaitEdge::Wait => {
            let [i, j] = [tail, head].map(|x| x.get_t().expect("t"));
            let [t1, t2] = [i, j].map(|x| p.transport_by_idx(x));
            format!("{dd_str}__wait__{}__{}", t_str(t1), t_str(t2))
        }
        AonWaitEdge::Exit => {
            let t = p.transport_by_idx(tail.get_t().expect("t"));
            let d = head.get_dd().expect("dd");
            let des = p.space_key(d.space());
            format!("{dd_str}__exit__{}__{des}_{}", t_str(t), d.time())
        }
        AonWaitEdge::Bypass(c) => {
            let com = p.commodity_by_idx(*c);
            format!("{dd_str}__bypass__{}", com.var_str(p))
        }
    }
}
