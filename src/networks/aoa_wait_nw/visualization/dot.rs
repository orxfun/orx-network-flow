use crate::graphs::visualization::dot::{
    DotGraph, EdgeSettings, VertexSettings, VertexShape, VertexStyle,
};
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, Vertex};
use crate::mcnf::Path;
use crate::networks::aoa_wait_nw::{AoaWaitEdge, AoaWaitGraph, AoaWaitNw};
use crate::{Commodity, CommodityData, FlowUnit, McnfSolution, Problem, Space, SpaceTime, Variant};
use alloc::string::{String, ToString};
use alloc::{format, vec::Vec};
use orx_iterable::Collection;

const EDGE_WIDTH_UNIFORM: f64 = 1.4;
const EDGE_WIDTH_WITH_FLOW: f64 = 2.8;
const EDGE_WIDTH_WITHOUT_FLOW: f64 = 0.8;

pub struct AoaWaitDotSettings {
    space_time: VertexSettings,
    wait: EdgeSettings,
    transport: EdgeSettings,
    bypass: EdgeSettings,
}

impl Default for AoaWaitDotSettings {
    fn default() -> Self {
        Self {
            space_time: VertexSettings {
                shape: Some(VertexShape::Circle),
                style: Some(VertexStyle::Filled),
                fill_color: Some(String::from("lightblue")),
            },
            wait: EdgeSettings {
                color: Some(String::from("lightgray")),
                pen_width: Some(EDGE_WIDTH_UNIFORM),
            },
            transport: EdgeSettings {
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

pub struct AoaWaitDot<'a, V>
where
    V: Variant,
{
    nw: &'a AoaWaitNw<'a, V>,
    settings: AoaWaitDotSettings,
    edge_settings_by_edge: Option<VecEdge<EdgeSettings>>,
    solution: Option<&'a McnfSolution<V>>,
}

impl<'a, V> AoaWaitDot<'a, V>
where
    V: Variant,
{
    pub fn new(nw: &'a AoaWaitNw<'a, V>, settings: Option<AoaWaitDotSettings>) -> Self {
        Self {
            nw,
            settings: settings.unwrap_or_default(),
            edge_settings_by_edge: None,
            solution: None,
        }
    }

    pub fn with_solution(mut self, solution: &'a McnfSolution<V>) -> Self {
        self.edge_settings_by_edge = Some(self.edge_settings_with_solution(solution));
        self.solution = Some(solution);
        self
    }

    fn edge_settings_default(&self, e: EIdx) -> &EdgeSettings {
        match self.graph().edge(e).data() {
            AoaWaitEdge::Wait => &self.settings.wait,
            AoaWaitEdge::Transport(_) => &self.settings.transport,
            AoaWaitEdge::Bypass(_) => &self.settings.bypass,
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
            AoaWaitEdge::Bypass(c) => {
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
            AoaWaitEdge::Transport(t) => {
                let all_path_flows = solution
                    .commodity_paths()
                    .iter()
                    .flat_map(|paths| paths.path_flows.iter());
                let matching = all_path_flows.filter(|pf| pf.path.as_slice().contains(t));
                FlowUnit::sum(matching.map(|pf| pf.flow))
            }
            AoaWaitEdge::Wait => {
                let tail_st = self.graph().vertex(edge.tail()).data().0;
                let head_st = self.graph().vertex(edge.head()).data().0;
                let matching =
                    solution
                        .commodity_paths()
                        .enumerated_iter()
                        .flat_map(|(c, paths)| {
                            let commodity = p.commodity_by_idx(c);
                            paths.path_flows.iter().filter(move |pf| {
                                uses_wait_arc(p, commodity, &pf.path, tail_st, head_st)
                            })
                        });
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
            AoaWaitEdge::Bypass(c) => {
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
            AoaWaitEdge::Transport(t) => solution
                .commodity_paths()
                .enumerated_iter()
                .filter_map(|(c, paths)| {
                    let flow = FlowUnit::sum(
                        paths
                            .path_flows
                            .iter()
                            .filter(|pf| pf.flow.is_pos() && pf.path.as_slice().contains(t))
                            .map(|pf| pf.flow),
                    );

                    if flow.is_pos() {
                        Some((commodity_short_str(p, c), flow))
                    } else {
                        None
                    }
                })
                .collect(),
            AoaWaitEdge::Wait => {
                let tail_st = self.graph().vertex(edge.tail()).data().0;
                let head_st = self.graph().vertex(edge.head()).data().0;

                solution
                    .commodity_paths()
                    .enumerated_iter()
                    .filter_map(|(c, paths)| {
                        let commodity = p.commodity_by_idx(c);
                        let flow = FlowUnit::sum(
                            paths
                                .path_flows
                                .iter()
                                .filter(|pf| {
                                    pf.flow.is_pos()
                                        && uses_wait_arc(p, commodity, &pf.path, tail_st, head_st)
                                })
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

    fn append_wait_vertices_between(
        &self,
        from: SpaceTime,
        to: SpaceTime,
        vertices: &mut Vec<VIdx>,
    ) {
        if from == to {
            return;
        }

        let st_to_v = self.nw.st_to_v();

        if from.space() != to.space() || from.time() > to.time() {
            if let Some(&v) = st_to_v.get(&to)
                && vertices.last() != Some(&v)
            {
                vertices.push(v);
            }
            return;
        }

        let mut times: Vec<_> = st_to_v
            .keys()
            .filter(|st| st.space() == from.space())
            .map(|st| st.time())
            .filter(|&t| t > from.time() && t <= to.time())
            .collect();
        times.sort_unstable();
        times.dedup();

        for time in times {
            let st = SpaceTime::new(from.space(), time);
            if let Some(&v) = st_to_v.get(&st)
                && vertices.last() != Some(&v)
            {
                vertices.push(v);
            }
        }
    }

    fn path_with_vertices_str(&self, c: Commodity, path: &Path) -> String {
        let p = self.nw.p();
        let commodity = p.commodity_by_idx(c);
        let st_to_v = self.nw.st_to_v();

        let mut vertices = Vec::new();
        let source = st_to_v[&commodity.origin()];
        vertices.push(source);

        let mut current = commodity.origin();

        for transport in path.as_slice() {
            let td = p.transport_by_idx(*transport);

            self.append_wait_vertices_between(current, td.origin(), &mut vertices);

            let head = st_to_v[&td.destination()];
            if vertices.last() != Some(&head) {
                vertices.push(head);
            }

            current = td.destination();
        }

        self.append_wait_vertices_between(current, commodity.destination(), &mut vertices);

        if let Some(&sink) = st_to_v.get(&commodity.destination())
            && vertices.last() != Some(&sink)
        {
            vertices.push(sink);
        }

        vertices
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("-")
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
                    self.path_with_vertices_str(commodity, &path_flow.path),
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

impl<'a, V> DotGraph for AoaWaitDot<'a, V>
where
    V: Variant,
{
    type G = AoaWaitGraph;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        let st = self.graph().vertex(v).data().0;

        let outgoing = self
            .graph()
            .vertex(v)
            .out_edges()
            .filter_map(|e| self.graph().edge(e).data().get_transport())
            .count();

        let incoming = self
            .graph()
            .vertex(v)
            .in_edges()
            .filter_map(|e| self.graph().edge(e).data().get_transport())
            .count();

        format!(
            "{}\n{}-{}\nout:{} in:{}",
            v,
            self.space(st.space()),
            st.time(),
            outgoing,
            incoming
        )
    }

    fn vertex_tooltip(&self, v: VIdx) -> Option<impl core::fmt::Display> {
        Some({
            let p = self.nw.p();
            let com_str = |(c, x): (Commodity, &CommodityData<V>)| com_str(p, c, x);
            let st = self.graph().vertex(v).data().0;

            let supply_coms = p.sorted_ro_commodities.value_by_key(&st);
            let demand_coms = p.sorted_dd_commodities.value_by_key(&st);

            match (supply_coms, demand_coms) {
                (Some(supply), Some(demand)) => {
                    let supply_total =
                        FlowUnit::sum(supply.iter().map(|&c| p.commodity_by_idx(c).amount()));
                    let demand_total =
                        FlowUnit::sum(demand.iter().map(|&c| p.commodity_by_idx(c).amount()));

                    let supply_lines: Vec<_> = supply
                        .iter()
                        .map(|&c| (c, p.commodity_by_idx(c)))
                        .map(com_str)
                        .collect();
                    let demand_lines: Vec<_> = demand
                        .iter()
                        .map(|&c| (c, p.commodity_by_idx(c)))
                        .map(com_str)
                        .collect();

                    format!(
                        "Source & sink vertex at {}-{}\n\
                         source commodities = {}\n\
                         total amount entering = {}\n\n\
                         {}\n\n\
                         sink commodities = {}\n\
                         total amount leaving = {}\n\n\
                         {}",
                        p.space_key(st.space()),
                        st.time(),
                        supply.len(),
                        supply_total,
                        supply_lines.join("\n"),
                        demand.len(),
                        demand_total,
                        demand_lines.join("\n")
                    )
                }
                (Some(commodities), None) => {
                    let total_amount =
                        FlowUnit::sum(commodities.iter().map(|&c| p.commodity_by_idx(c).amount()));
                    let commodities: Vec<_> = commodities
                        .iter()
                        .map(|&c| (c, p.commodity_by_idx(c)))
                        .map(com_str)
                        .collect();

                    format!(
                        "Source vertex per origin & ready\n{} commodities\ntotal amount entering = {}\n\n{}",
                        commodities.len(),
                        total_amount,
                        commodities.join("\n")
                    )
                }
                (None, Some(commodities)) => {
                    let total_amount =
                        FlowUnit::sum(commodities.iter().map(|&c| p.commodity_by_idx(c).amount()));
                    let commodities: Vec<_> = commodities
                        .iter()
                        .map(|&c| (c, p.commodity_by_idx(c)))
                        .map(com_str)
                        .collect();

                    format!(
                        "Sink vertex per destination & due time\n{} commodities\ntotal amount leaving = {}\n\n{}",
                        commodities.len(),
                        total_amount,
                        commodities.join("\n")
                    )
                }
                (None, None) => {
                    format!("space-time node\n{}-{}", p.space_key(st.space()), st.time())
                }
            }
        })
    }

    fn vertex_settings(&self, _v: VIdx) -> &VertexSettings {
        &self.settings.space_time
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
            None => String::new(),
        }
    }

    fn edge_tooltip(&self, e: EIdx) -> Option<impl core::fmt::Display> {
        let p = self.nw.p();
        let edge = self.graph().edge(e);
        let tail = self.graph().vertex(edge.tail()).data().0;
        let head = self.graph().vertex(edge.head()).data().0;

        let base = match edge.data() {
            AoaWaitEdge::Wait => {
                let s = p.space_key(tail.space());
                format!("Wait arc at {s}: {} -> {}", tail.time(), head.time())
            }
            AoaWaitEdge::Transport(t) => {
                let td = p.transport_by_idx(*t);
                let [o, d] = [
                    p.space_key(td.origin().space()),
                    p.space_key(td.destination().space()),
                ];
                let [dt, at] = [td.origin().time(), td.destination().time()];
                let cap = td.capacity();
                format!("Transport arc {t}\n{o}-{d} at {dt}-{at}\ncapacity={cap}")
            }
            AoaWaitEdge::Bypass(c) => {
                let com = p.commodity_by_idx(*c);
                let com_str = com_str(p, *c, com);
                format!("Bypass arc with lost revenue cost\n{com_str}")
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

                        format!("{base}\n\nCommodities on arc:\n{}", lines.join("\n"))
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

fn uses_wait_arc<V: Variant>(
    p: &Problem<V>,
    commodity: &CommodityData<V>,
    path: &Path,
    tail: SpaceTime,
    head: SpaceTime,
) -> bool {
    if tail.space() != head.space() || tail.time() >= head.time() {
        return false;
    }

    let transports = path.as_slice();
    if transports.is_empty() {
        return false;
    }

    if let Some(&first_t) = transports.first() {
        let first = p.transport_by_idx(first_t);
        if commodity.origin().space() == tail.space()
            && first.origin().space() == head.space()
            && commodity.origin().time() <= tail.time()
            && head.time() <= first.origin().time()
        {
            return true;
        }
    }

    for w in transports.windows(2) {
        let t1 = p.transport_by_idx(w[0]);
        let t2 = p.transport_by_idx(w[1]);

        let d1 = t1.destination();
        let o2 = t2.origin();
        if d1.space() == tail.space()
            && o2.space() == head.space()
            && d1.time() <= tail.time()
            && head.time() <= o2.time()
        {
            return true;
        }
    }

    if let Some(&last_t) = transports.last() {
        let last = p.transport_by_idx(last_t);
        if last.destination().space() == tail.space()
            && commodity.destination().space() == head.space()
            && last.destination().time() <= tail.time()
            && head.time() <= commodity.destination().time()
        {
            return true;
        }
    }

    false
}

fn escape_dot_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
