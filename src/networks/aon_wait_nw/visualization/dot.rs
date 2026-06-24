use crate::graphs::visualization::dot::{
    DotGraph, EdgeSettings, VertexSettings, VertexShape, VertexStyle,
};
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, Vertex};
use crate::mcnf::Path;
use crate::networks::aon_wait_nw::{AonWaitEdge, AonWaitGraph, AonWaitNw, AonWaitVertex};
use crate::{
    Commodity, CommodityData, FlowUnit, McnfSolution, Problem, Space, SpaceTime, Transport, Variant,
};
use alloc::string::{String, ToString};
use alloc::{format, vec::Vec};
use orx_iterable::{Collection, Iterable};

const EDGE_WIDTH_UNIFORM: f64 = 1.4;
const EDGE_WIDTH_WITH_FLOW: f64 = 2.8;
const EDGE_WIDTH_WITHOUT_FLOW: f64 = 0.8;

pub struct AonWaitDotSettings {
    transport: VertexSettings,
    ready_ori: VertexSettings,
    due_des: VertexSettings,
    wait: EdgeSettings,
    connect: EdgeSettings,
    enter: EdgeSettings,
    exit: EdgeSettings,
    bypass: EdgeSettings,
}

impl Default for AonWaitDotSettings {
    fn default() -> Self {
        Self {
            transport: VertexSettings {
                shape: Some(VertexShape::Rect),
                style: None,
                fill_color: None,
            },
            ready_ori: VertexSettings {
                shape: Some(VertexShape::Circle),
                style: Some(VertexStyle::Filled),
                fill_color: Some(String::from("lightgreen")),
            },
            due_des: VertexSettings {
                shape: Some(VertexShape::Circle),
                style: Some(VertexStyle::Filled),
                fill_color: Some(String::from("tomato")),
            },
            wait: EdgeSettings {
                color: Some(String::from("lightgray")),
                pen_width: Some(EDGE_WIDTH_UNIFORM),
            },
            connect: EdgeSettings {
                color: Some(String::from("darkgreen")),
                pen_width: Some(EDGE_WIDTH_UNIFORM),
            },
            enter: EdgeSettings {
                color: Some(String::from("lightgray")),
                pen_width: Some(EDGE_WIDTH_UNIFORM),
            },
            exit: EdgeSettings {
                color: Some(String::from("darkgreen")),
                pen_width: Some(EDGE_WIDTH_UNIFORM),
            },
            bypass: EdgeSettings {
                color: Some(String::from("orange")),
                pen_width: Some(EDGE_WIDTH_UNIFORM),
            },
        }
    }
}

pub struct AonWaitDot<'a, V>
where
    V: Variant,
{
    nw: &'a AonWaitNw<'a, V>,
    settings: AonWaitDotSettings,
    edge_settings_by_edge: Option<VecEdge<EdgeSettings>>,
    flows_deprecated: Option<&'a VecEdge<V::F>>,
    solution: Option<&'a McnfSolution<V>>,
}

impl<'a, V> AonWaitDot<'a, V>
where
    V: Variant,
{
    pub fn new(nw: &'a AonWaitNw<'a, V>, settings: Option<AonWaitDotSettings>) -> Self {
        Self {
            nw,
            settings: settings.unwrap_or_default(),
            edge_settings_by_edge: None,
            flows_deprecated: None,
            solution: None,
        }
    }

    pub fn with_flows_deprecated(mut self, flows: &'a VecEdge<V::F>) -> Self {
        self.flows_deprecated = Some(flows);
        self
    }

    pub fn with_solution(mut self, solution: &'a McnfSolution<V>) -> Self {
        self.edge_settings_by_edge = Some(self.edge_settings_with_solution(solution));
        self.solution = Some(solution);
        self
    }

    fn edge_settings_default(&self, e: EIdx) -> &EdgeSettings {
        match self.graph().edge(e).data() {
            AonWaitEdge::Wait => &self.settings.wait,
            AonWaitEdge::Connect => &self.settings.connect,
            AonWaitEdge::Enter => &self.settings.enter,
            AonWaitEdge::Exit => &self.settings.exit,
            AonWaitEdge::Bypass(_) => &self.settings.bypass,
        }
    }

    fn edge_settings_with_solution(&self, solution: &McnfSolution<V>) -> VecEdge<EdgeSettings> {
        let mut settings_by_edge = VecEdge::new();

        for e in self.graph().edge_indices() {
            let mut settings = self.edge_settings_default(e).clone();
            let flow = self.edge_flow_from_solution(e, solution);
            settings.pen_width = match flow.is_pos() {
                true => Some(EDGE_WIDTH_WITH_FLOW),
                false => Some(EDGE_WIDTH_WITHOUT_FLOW),
            };
            settings_by_edge.push(settings);
        }

        settings_by_edge
    }

    fn space(&self, space: Space) -> &V::S {
        self.nw.p().space_key(space)
    }

    fn edge_flow_from_solution(&self, e: EIdx, solution: &McnfSolution<V>) -> V::F {
        let p = self.nw.p();
        let edge = self.graph().edge(e);

        match edge.data() {
            AonWaitEdge::Bypass(c) => {
                let amount = p.commodity_by_idx(*c).amount();
                let served = FlowUnit::sum(
                    solution.commodity_paths()[*c]
                        .path_flows
                        .iter()
                        .map(|x| x.flow),
                );
                match amount > served {
                    true => amount - served,
                    false => FlowUnit::zero(),
                }
            }
            AonWaitEdge::Enter => {
                let ro = self
                    .graph()
                    .vertex(edge.tail())
                    .data()
                    .get_ro()
                    .expect("ro");
                let t = self.graph().vertex(edge.head()).data().get_t().expect("t");
                let paths = solution.commodity_paths().enumerated_iter();
                let matching = paths.filter(|(c, _)| p.commodity_by_idx(*c).origin() == ro);
                let matching = matching.flat_map(|(_, paths)| paths.path_flows.iter());
                let matching = matching.filter(|pf| pf.path.first() == Some(t));
                FlowUnit::sum(matching.map(|pf| pf.flow))
            }
            AonWaitEdge::Exit => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let dd = self
                    .graph()
                    .vertex(edge.head())
                    .data()
                    .get_dd()
                    .expect("dd");
                let paths = solution.commodity_paths().enumerated_iter();
                let matching = paths.filter(|(c, _)| p.commodity_by_idx(*c).destination() == dd);
                let matching = matching.flat_map(|(_, paths)| paths.path_flows.iter());
                let matching = matching.filter(|pf| pf.path.last() == Some(t));
                FlowUnit::sum(matching.map(|pf| pf.flow))
            }
            AonWaitEdge::Wait | AonWaitEdge::Connect => {
                let t1 = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t2 = self.graph().vertex(edge.head()).data().get_t().expect("t");
                let all_path_flows = solution
                    .commodity_paths()
                    .iter()
                    .flat_map(|paths| paths.path_flows.iter());
                let matching = all_path_flows.filter(|pf| has_transition(&pf.path, t1, t2));
                FlowUnit::sum(matching.map(|pf| pf.flow))
            }
        }
    }

    fn edge_commodity_flows_from_solution(
        &self,
        e: EIdx,
        solution: &McnfSolution<V>,
    ) -> Vec<(String, V::F)> {
        let p = self.nw.p();
        let edge = self.graph().edge(e);

        match edge.data() {
            AonWaitEdge::Bypass(c) => {
                let amount = p.commodity_by_idx(*c).amount();
                let served = FlowUnit::sum(
                    solution.commodity_paths()[*c]
                        .path_flows
                        .iter()
                        .map(|x| x.flow),
                );

                if amount > served {
                    Vec::from([(commodity_short_str(p, *c), amount - served)])
                } else {
                    Vec::new()
                }
            }
            AonWaitEdge::Enter => {
                let ro = self
                    .graph()
                    .vertex(edge.tail())
                    .data()
                    .get_ro()
                    .expect("ro");
                let t = self.graph().vertex(edge.head()).data().get_t().expect("t");

                solution
                    .commodity_paths()
                    .enumerated_iter()
                    .filter_map(|(c, paths)| {
                        if p.commodity_by_idx(c).origin() != ro {
                            return None;
                        }

                        let flow = FlowUnit::sum(
                            paths
                                .path_flows
                                .iter()
                                .filter(|pf| pf.flow.is_pos() && pf.path.first() == Some(t))
                                .map(|pf| pf.flow),
                        );

                        if flow.is_pos() {
                            Some((commodity_short_str(p, c), flow))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            AonWaitEdge::Exit => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let dd = self
                    .graph()
                    .vertex(edge.head())
                    .data()
                    .get_dd()
                    .expect("dd");

                solution
                    .commodity_paths()
                    .enumerated_iter()
                    .filter_map(|(c, paths)| {
                        if p.commodity_by_idx(c).destination() != dd {
                            return None;
                        }

                        let flow = FlowUnit::sum(
                            paths
                                .path_flows
                                .iter()
                                .filter(|pf| pf.flow.is_pos() && pf.path.last() == Some(t))
                                .map(|pf| pf.flow),
                        );

                        if flow.is_pos() {
                            Some((commodity_short_str(p, c), flow))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            AonWaitEdge::Wait | AonWaitEdge::Connect => {
                let t1 = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t2 = self.graph().vertex(edge.head()).data().get_t().expect("t");

                solution
                    .commodity_paths()
                    .enumerated_iter()
                    .filter_map(|(c, paths)| {
                        let flow = FlowUnit::sum(
                            paths
                                .path_flows
                                .iter()
                                .filter(|pf| pf.flow.is_pos() && has_transition(&pf.path, t1, t2))
                                .map(|pf| pf.flow),
                        );

                        if flow.is_pos() {
                            Some((commodity_short_str(p, c), flow))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        }
    }

    fn path_used_transports_str(&self, path: &Path) -> String {
        let p = self.nw.p();
        path.used_transports(p)
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn path_with_endpoints_str(&self, c: Commodity, path: &Path) -> String {
        let p = self.nw.p();
        let commodity = p.commodity_by_idx(c);
        let source = self.nw.ro_to_v()[&commodity.origin()];
        let sink = self.nw.dd_to_v()[&commodity.destination()];

        format!("{source}-{}-{sink}", path)
    }

    fn graph_path_table_label_from_solution(&self, solution: &McnfSolution<V>) -> Option<String> {
        let p = self.nw.p();

        let mut rows = Vec::new();
        for (commodity, paths) in solution.commodity_paths().enumerated_iter() {
            let commodity_str = commodity_short_str(p, commodity);
            for path_flow in &paths.path_flows {
                if path_flow.flow.is_nonpos() {
                    continue;
                }
                rows.push((
                    commodity_str.clone(),
                    self.path_used_transports_str(&path_flow.path),
                    path_flow.path.to_str_as_spaces(p),
                    self.path_with_endpoints_str(commodity, &path_flow.path),
                    path_flow.flow.to_string(),
                ));
            }
        }

        if rows.is_empty() {
            return None;
        }

        let mut table = String::from(
            "<TABLE BORDER=\"1\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"4\">",
        );
        table.push_str(
            "<TR><TD BGCOLOR=\"#f2f2f2\"><B>Commodity</B></TD><TD BGCOLOR=\"#f2f2f2\"><B>Transports</B></TD><TD BGCOLOR=\"#f2f2f2\"><B>Locations</B></TD><TD BGCOLOR=\"#f2f2f2\"><B>Path</B></TD><TD BGCOLOR=\"#f2f2f2\"><B>Flow</B></TD></TR>",
        );

        for (commodity, transports, locations, path_with_waiting, flow) in rows {
            let commodity = escape_dot_html(&commodity);
            let transports = escape_dot_html(&transports);
            let locations = escape_dot_html(&locations);
            let path_with_waiting = escape_dot_html(&path_with_waiting);
            let flow = escape_dot_html(&flow);
            table.push_str(&format!(
                "<TR><TD ALIGN=\"LEFT\">{commodity}</TD><TD ALIGN=\"LEFT\">{transports}</TD><TD ALIGN=\"LEFT\">{locations}</TD><TD ALIGN=\"LEFT\">{path_with_waiting}</TD><TD ALIGN=\"RIGHT\">{flow}</TD></TR>"
            ));
        }

        table.push_str("</TABLE>");
        Some(table)
    }
}

impl<'a, V> DotGraph for AonWaitDot<'a, V>
where
    V: Variant,
{
    type G = AonWaitGraph;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        let p = self.nw.p();
        match self.graph().vertex(v).data() {
            AonWaitVertex::Transport(t) => {
                let data = p.transport_by_idx(*t);
                format!(
                    "{}\n{}-{}\n{}-{}",
                    t,
                    self.space(data.origin().space()),
                    self.space(data.destination().space()),
                    data.origin().time(),
                    data.destination().time()
                )
            }
            AonWaitVertex::ReadyOri(ro) => {
                let commodities = p.sorted_ro_commodities.value_by_key_unc(ro);
                let amounts = commodities.iter().map(|&c| p.commodity_by_idx(c).amount());
                let total_amount = FlowUnit::sum(amounts);
                let ori = p.space_key(ro.space());
                format!("{}\n{}-{}\n+{total_amount}", v, ori, ro.time())
            }
            AonWaitVertex::DueDes(dd) => {
                let commodities = p.sorted_dd_commodities.value_by_key_unc(dd);
                let amounts = commodities.iter().map(|&c| p.commodity_by_idx(c).amount());
                let total_amount = FlowUnit::sum(amounts);
                let des = p.space_key(dd.space());
                format!("{}\n{}-{}\n-{total_amount}", v, des, dd.time())
            }
        }
    }

    fn vertex_tooltip(&self, v: VIdx) -> Option<impl core::fmt::Display> {
        Some({
            let p = self.nw.p();
            let com_str = |(c, x): (Commodity, &CommodityData<V>)| com_str(p, c, x);

            match self.graph().vertex(v).data() {
                AonWaitVertex::Transport(t) => {
                    let capacity = p.transport_by_idx(*t).capacity();
                    format!("transport capacity = {capacity}")
                }
                AonWaitVertex::ReadyOri(ro) => {
                    let commodities = p.sorted_ro_commodities.value_by_key_unc(ro);
                    let num_commodities = commodities.len();
                    let commodities = commodities.as_iterable();
                    let commodities = commodities.mapped(|&c| (c, p.commodity_by_idx(c)));
                    let total_amount = FlowUnit::sum(commodities.iter().map(|x| x.1.amount()));
                    let commodities: Vec<_> = commodities.iter().map(com_str).collect();
                    let commodities = commodities.join("\n");
                    format!(
                        "Source vertex per origin & ready\n{num_commodities} commodities\ntotal amount entering = {total_amount}\n\n{commodities}"
                    )
                }
                AonWaitVertex::DueDes(dd) => {
                    let commodities = p.sorted_dd_commodities.value_by_key_unc(dd);
                    let num_commodities = commodities.len();
                    let commodities = commodities.as_iterable();
                    let commodities = commodities.mapped(|&c| (c, p.commodity_by_idx(c)));
                    let total_amount = FlowUnit::sum(commodities.iter().map(|x| x.1.amount()));
                    let commodities: Vec<_> = commodities.iter().map(com_str).collect();
                    let commodities = commodities.join("\n");
                    format!(
                        "Sink vertex per destination & due time\n{num_commodities} commodities\ntotal amount leaving = {total_amount}\n\n{commodities}"
                    )
                }
            }
        })
    }

    fn vertex_settings(&self, v: VIdx) -> &VertexSettings {
        match self.graph().vertex(v).data() {
            AonWaitVertex::Transport(_) => &self.settings.transport,
            AonWaitVertex::ReadyOri(_) => &self.settings.ready_ori,
            AonWaitVertex::DueDes(_) => &self.settings.due_des,
            _ => todo!("vertex settings"),
        }
    }

    fn edge_label(&self, e: EIdx) -> impl core::fmt::Display {
        match &self.solution {
            Some(solution) => {
                let flow = self.edge_flow_from_solution(e, solution);
                match flow.is_pos() {
                    true => flow.to_string(),
                    false => String::new(),
                }
            }
            None => match &self.flows_deprecated {
                Some(flows) => flows[e].to_string(),
                None => String::new(),
            },
        }
    }

    fn edge_tooltip(&self, e: EIdx) -> Option<impl core::fmt::Display> {
        let p = self.nw.p();
        let edge = self.graph().edge(e);
        let space = |st: SpaceTime| self.nw.p().space_key(st.space());

        let base = match edge.data() {
            AonWaitEdge::Wait => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t = p.transport_by_idx(t);
                let [o, d] = [space(t.origin()), space(t.destination())];
                format!("Waiting edge at {o} among {o}-{d} transports")
            }
            AonWaitEdge::Connect => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t = p.transport_by_idx(t);
                let [o, d] = [space(t.origin()), space(t.destination())];
                let [dt, at] = [t.origin().time(), t.destination().time()];
                format!("Transport edge using capacity of\n{o}-{d} at {dt}-{at}")
            }
            AonWaitEdge::Enter => format!("Entering transport network"),
            AonWaitEdge::Exit => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t = p.transport_by_idx(t);
                let [o, d] = [space(t.origin()), space(t.destination())];
                let [dt, at] = [t.origin().time(), t.destination().time()];
                format!("Transport edge using capacity of\n{o}-{d} at {dt}-{at}")
            }
            AonWaitEdge::Bypass(c) => {
                let com = p.commodity_by_idx(*c);
                let com_str = com_str(p, *c, &com);
                format!("Bypass edge with lost revenue cost\n{com_str}")
            }
        };

        Some(match self.solution {
            Some(solution) => {
                let flow = self.edge_flow_from_solution(e, solution);
                if flow.is_nonpos() {
                    base
                } else {
                    let commodity_flows = self.edge_commodity_flows_from_solution(e, solution);
                    if commodity_flows.is_empty() {
                        base
                    } else {
                        let left_header = "Commodity";
                        let right_header = "Flow";
                        let left_width = core::cmp::max(
                            left_header.len(),
                            commodity_flows
                                .iter()
                                .map(|(commodity, _)| commodity.len())
                                .max()
                                .unwrap_or(0),
                        );
                        let right_values: Vec<String> =
                            commodity_flows.iter().map(|(_, f)| f.to_string()).collect();
                        let right_width = core::cmp::max(
                            right_header.len(),
                            right_values.iter().map(|x| x.len()).max().unwrap_or(0),
                        );

                        let mut lines = Vec::with_capacity(2 + commodity_flows.len());
                        lines.push(format!(
                            "{:<left_width$}  {:>right_width$}",
                            left_header, right_header
                        ));
                        for ((commodity, _), flow_str) in
                            commodity_flows.iter().zip(right_values.iter())
                        {
                            lines.push(format!(
                                "{:<left_width$}  {:>right_width$}",
                                commodity, flow_str
                            ));
                        }

                        format!("{base}\n\nCommodities on edge:\n{}", lines.join("\n"))
                    }
                }
            }
            None => base,
        })
    }

    fn edge_settings(&self, e: EIdx) -> &EdgeSettings {
        match &self.edge_settings_by_edge {
            Some(settings) => &settings[e],
            None => self.edge_settings_default(e),
        }
    }

    fn graph_label(&self) -> Option<impl core::fmt::Display> {
        self.solution
            .and_then(|solution| self.graph_path_table_label_from_solution(solution))
    }

    fn graph(&self) -> &Self::G {
        self.nw.g()
    }

    fn dot_string(&self) -> String {
        let mut dot = String::from("digraph G {\n");

        if let Some(graph_label) = self.graph_label() {
            dot.push_str("    labelloc=\"b\";\n");
            dot.push_str("    labeljust=\"l\";\n");
            dot.push_str(&format!("    label=<{}>;\n", graph_label));
        }

        for v in self.vertices() {
            let label = self.vertex_label(v);
            let tooltip = self.vertex_tooltip(v);
            let settings = self.vertex_settings(v);

            let vertex = match tooltip {
                Some(tooltip) => {
                    format!(
                        "    {} [label=\"{label}\"{settings} tooltip=\"{}\"]",
                        v, tooltip
                    )
                }
                None => format!("    {} [label=\"{label}\"{settings}]", v),
            };

            dot.push_str(&vertex);
            dot.push_str(";\n");
        }

        for (e, tail, head) in self.edges() {
            let label = self.edge_label(e);
            let tooltip = self.edge_tooltip(e);
            let settings = self.edge_settings(e);
            let edge = match tooltip {
                Some(tooltip) => {
                    format!(
                        "    {} -> {} [label=\"{}\" {} tooltip=\"{}\"]",
                        tail, head, label, settings, tooltip
                    )
                }
                None => format!(
                    "    {} -> {} [label=\"{}\" {}]",
                    tail, head, label, settings
                ),
            };
            dot.push_str(&edge);
            dot.push_str(";\n");
        }

        dot.push('}');

        dot
    }
}

fn com_str<V: Variant>(p: &Problem<V>, c: Commodity, data: &CommodityData<V>) -> String {
    let s = |s: Space| p.space_key(s);
    format!(
        "commodity {}-{} | amount={} | revenue={}",
        s(data.origin().space()),
        s(data.destination().space()),
        data.amount(),
        p.costs.lost_revenue.cost(c)
    )
}

fn commodity_short_str<V: Variant>(p: &Problem<V>, c: Commodity) -> String {
    let data = p.commodity_by_idx(c);
    let s = |s: Space| p.space_key(s);
    format!(
        "{}-{} {}-{}",
        s(data.origin().space()),
        s(data.destination().space()),
        data.origin().time(),
        data.destination().time()
    )
}

fn has_transition(path: &Path, tail: Transport, head: Transport) -> bool {
    let has_in_slice = |slice: &[Transport]| slice.windows(2).any(|w| w[0] == tail && w[1] == head);

    match path {
        Path::OneLeg(_) => false,
        Path::TwoLegs(path) => has_in_slice(path),
        Path::ThreeLegs(path) => has_in_slice(path),
        Path::Long(path) => has_in_slice(path),
    }
}

fn escape_dot_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
