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
    teleport_settings: NodeSettings,
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
        let mut teleport_settings = settings.transport.clone();
        teleport_settings.style = Some(NodeStyle::Dotted);

        Self {
            problem,
            network,
            settings,
            teleport_settings,
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
            AonVertex::Teleport(c) => {
                let commodity = prob.commodity_by_idx(*c);
                let ori = prob.space_key(commodity.origin().space());
                let des = prob.space_key(commodity.destination().space());
                let rt = commodity.origin().time();
                let due = commodity.destination().time();
                format!("{} : c{}\n{}-{}\n{}-{}", v, c, ori, des, rt, due)
            }
        }
    }

    fn vertex_tooltip(&self, _: VIdx, vertex: &Vertex<Self::V>) -> Option<String> {
        let prob = self.problem;
        let nw = self.network;
        let tooltip = match vertex.data() {
            AonVertex::Source(s) => {
                let source = nw.source(*s);
                let commodities = source
                    .commodities()
                    .iter()
                    .map(|&c| (prob.commodity_key(c), prob.commodity_by_idx(c)))
                    .into_iterable();
                let num_commodities = commodities.iter().len();
                let total_amount = FlowUnit::sum(commodities.iter().map(|x| x.1.amount()));
                let keys: Vec<_> = commodities.iter().map(|x| x.0.to_string()).collect();
                let keys = keys.join("\n");
                format!(
                    "total amount = {}\n# commodities = {}:\n{}",
                    total_amount, num_commodities, keys
                )
            }
            AonVertex::Sink(t) => {
                // let st = nw.sink_st(*t);
                // let space = prob.space_key(st.space());
                // let time = st.time();
                // format!("{} : t{}\n{}-{}", v, t, space, time)
                String::new()
            }
            AonVertex::Transport(t) => {
                // let transport = prob.transport_by_idx(*t);
                // let ori = prob.space_key(transport.origin().space());
                // let des = prob.space_key(transport.destination().space());
                // let dt = transport.origin().time();
                // let at = transport.destination().time();
                // format!("{}\n{}-{}\n{}-{}", v, ori, des, dt, at)
                String::new()
            }
            AonVertex::Teleport(c) => {
                // let commodity = prob.commodity_by_idx(*c);
                // let ori = prob.space_key(commodity.origin().space());
                // let des = prob.space_key(commodity.destination().space());
                // let rt = commodity.origin().time();
                // let due = commodity.destination().time();
                // format!("{} : c{}\n{}-{}\n{}-{}", v, c, ori, des, rt, due)
                String::new()
            }
        };

        Some(tooltip)
    }

    fn vertex_settings(&self, _: VIdx, vertex: &Vertex<Self::V>) -> String {
        match vertex.data() {
            AonVertex::Source(_) => self.settings.source.to_string(),
            AonVertex::Sink(_) => self.settings.sink.to_string(),
            AonVertex::Transport(_) => self.settings.transport.to_string(),
            AonVertex::Teleport(_) => self.teleport_settings.to_string(),
        }
    }
}
