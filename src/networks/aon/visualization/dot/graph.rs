use crate::commodities::CommodityData;
use crate::flow_units::FlowUnit;
use crate::graph::visualization::dot::{DotGraph, NodeSettings, NodeStyle};
use crate::graph::{VIdx, Vertex};
use crate::networks::aon::visualization::dot::settings::AonDotGraphSettings;
use crate::networks::aon::{AonEdge, AonVertex};
use crate::{AonNetwork, Graph, Problem, Variant};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use orx_iterable::{IntoCloningIterable, Iterable};

pub struct AonDotGraph<'a, V: Variant> {
    problem: &'a Problem<V>,
    network: &'a AonNetwork<'a, V>,
    settings: AonDotGraphSettings,
}

impl<'a, V: Variant> AonDotGraph<'a, V> {
    pub fn new(problem: &'a Problem<V>, network: &'a AonNetwork<'a, V>) -> Self {
        Self::with_settings(problem, network, Default::default())
    }

    pub fn with_settings(
        problem: &'a Problem<V>,
        network: &'a AonNetwork<'a, V>,
        settings: AonDotGraphSettings,
    ) -> Self {
        Self {
            problem,
            network,
            settings,
        }
    }
}

impl<V: Variant> DotGraph for AonDotGraph<'_, V> {
    type V = AonVertex;

    type E = AonEdge;

    fn graph(&self) -> &Graph<Self::V, Self::E> {
        self.network.graph()
    }

    fn vertex_label(&self, v: VIdx, vertex: &Vertex<Self::V>) -> String {
        let prob = self.problem;
        let nw = self.network;
        match vertex.data() {
            AonVertex::Source(s) => {
                let st = nw.source_st(*s);
                let space = prob.space_key(st.space());
                let time = st.time();
                format!("{} : s{}\n{}-{}", v, s, space, time)
            }
            AonVertex::Sink(t) => {
                let st = nw.sink_st(*t);
                let space = prob.space_key(st.space());
                let time = st.time();
                format!("{} : t{}\n{}-{}", v, t, space, time)
            }
            AonVertex::Transport(t) => {
                let transport = prob.transport_by_idx(*t);
                let ori = prob.space_key(transport.origin().space());
                let des = prob.space_key(transport.destination().space());
                let dt = transport.origin().time();
                let at = transport.destination().time();
                format!("{}\n{}-{}\n{}-{}", v, ori, des, dt, at)
            }
        }
    }

    fn vertex_tooltip(&self, _: VIdx, vertex: &Vertex<Self::V>) -> Option<String> {
        let p = self.problem;
        let nw = self.network;

        let commodity_info = |x: (&V::K, &CommodityData<V>)| {
            let ori = p.space_key(x.1.origin().space());
            let des = p.space_key(x.1.destination().space());
            let rt = x.1.origin().time();
            let due = x.1.destination().time();
            format!(
                "{}: {}-{} | {}-{} | {}",
                x.0,
                ori,
                des,
                rt,
                due,
                x.1.amount()
            )
        };
        match vertex.data() {
            AonVertex::Source(s) => {
                let source = nw.source(*s);
                let commodities = source
                    .commodities()
                    .iter()
                    .map(|&c| (p.commodity_key(c), p.commodity_by_idx(c)))
                    .into_iterable();
                let num_commodities = commodities.iter().len();
                let total_amount = FlowUnit::sum(commodities.iter().map(|x| x.1.amount()));
                let keys: Vec<_> = commodities.iter().map(commodity_info).collect();
                let keys = keys.join("\n");
                Some(format!(
                    "total amount = {}\n{} commodities:\n{}",
                    total_amount, num_commodities, keys
                ))
            }
            AonVertex::Sink(t) => {
                let sink = nw.sink(*t);
                let commodities = sink
                    .commodities()
                    .iter()
                    .map(|&c| (p.commodity_key(c), p.commodity_by_idx(c)))
                    .into_iterable();
                let num_commodities = commodities.iter().len();
                let total_amount = FlowUnit::sum(commodities.iter().map(|x| x.1.amount()));
                let keys: Vec<_> = commodities.iter().map(commodity_info).collect();
                let keys = keys.join("\n");
                Some(format!(
                    "total amount = {}\n{} commodities:\n{}",
                    total_amount, num_commodities, keys
                ))
            }
            AonVertex::Transport(t) => None,
        }
    }

    fn vertex_settings(&self, _: VIdx, vertex: &Vertex<Self::V>) -> String {
        match vertex.data() {
            AonVertex::Source(_) => self.settings.source.to_string(),
            AonVertex::Sink(_) => self.settings.sink.to_string(),
            AonVertex::Transport(_) => self.settings.transport.to_string(),
        }
    }
}
