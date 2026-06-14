use crate::flow_units::FlowUnit;
use crate::graphs::visualization::dot::{
    DotGraph, EdgeSettings, VertexSettings, VertexShape, VertexStyle,
};
use crate::graphs::{EIdx, Edge, Graph, VIdx, VecEdge, Vertex};
use crate::networks::conn_wait_nw::{ConnWaitEdge, ConnWaitGraph, ConnWaitNw, ConnWaitVertex};
use crate::{Commodity, CommodityData, Problem, Space, SpaceTime, Variant};
use alloc::string::{String, ToString};
use alloc::{format, vec::Vec};
use orx_iterable::{Collection, Iterable};

pub struct ConnWaitDotSettings {
    transport: VertexSettings,
    ready_ori: VertexSettings,
    due_des: VertexSettings,
    wait: EdgeSettings,
    connect: EdgeSettings,
    enter: EdgeSettings,
    exit: EdgeSettings,
    bypass: EdgeSettings,
}

impl Default for ConnWaitDotSettings {
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
            },
            connect: EdgeSettings {
                color: Some(String::from("darkgreen")),
            },
            enter: EdgeSettings {
                color: Some(String::from("lightgray")),
            },
            exit: EdgeSettings {
                color: Some(String::from("darkgreen")),
            },
            bypass: EdgeSettings {
                color: Some(String::from("orange")),
            },
        }
    }
}

pub struct ConnWaitDot<'a, V>
where
    V: Variant,
{
    nw: &'a ConnWaitNw<'a, V>,
    settings: ConnWaitDotSettings,
    flows: Option<&'a VecEdge<V::F>>,
}

impl<'a, V> ConnWaitDot<'a, V>
where
    V: Variant,
{
    pub fn new(nw: &'a ConnWaitNw<'a, V>, settings: Option<ConnWaitDotSettings>) -> Self {
        Self {
            nw,
            settings: settings.unwrap_or_default(),
            flows: None,
        }
    }

    pub fn with_flows(mut self, flows: &'a VecEdge<V::F>) -> Self {
        self.flows = Some(flows);
        self
    }

    fn space(&self, space: Space) -> &V::S {
        self.nw.p.space_key(space)
    }
}

impl<'a, V> DotGraph for ConnWaitDot<'a, V>
where
    V: Variant,
{
    type G = ConnWaitGraph;

    fn vertex_label(&self, v: VIdx) -> impl core::fmt::Display {
        let p = self.nw.p;
        match self.graph().vertex(v).data() {
            ConnWaitVertex::Transport(t) => {
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
            ConnWaitVertex::ReadyOri(ro) => {
                let commodities = p.sorted_ro_commodities2.value_by_key_unc(ro);
                let amounts = commodities.iter().map(|&c| p.commodity_by_idx(c).amount());
                let total_amount = FlowUnit::sum(amounts);
                let ori = p.space_key(ro.space());
                format!("{}\n{}-{}\n+{total_amount}", v, ori, ro.time())
            }
            ConnWaitVertex::DueDes(dd) => {
                let commodities = p.sorted_dd_commodities2.value_by_key_unc(dd);
                let amounts = commodities.iter().map(|&c| p.commodity_by_idx(c).amount());
                let total_amount = FlowUnit::sum(amounts);
                let des = p.space_key(dd.space());
                format!("{}\n{}-{}\n-{total_amount}", v, des, dd.time())
            }
        }
    }

    fn vertex_tooltip(&self, v: VIdx) -> Option<impl core::fmt::Display> {
        Some({
            let p = self.nw.p;
            let com_str = |(c, x): (Commodity, &CommodityData<V>)| com_str(p, c, x);

            match self.graph().vertex(v).data() {
                ConnWaitVertex::Transport(t) => {
                    let capacity = p.transport_by_idx(*t).capacity();
                    format!("transport capacity = {capacity}")
                }
                ConnWaitVertex::ReadyOri(ro) => {
                    let commodities = p.sorted_ro_commodities2.value_by_key_unc(ro);
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
                ConnWaitVertex::DueDes(dd) => {
                    let commodities = p.sorted_dd_commodities2.value_by_key_unc(dd);
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
            ConnWaitVertex::Transport(_) => &self.settings.transport,
            ConnWaitVertex::ReadyOri(_) => &self.settings.ready_ori,
            ConnWaitVertex::DueDes(_) => &self.settings.due_des,
            _ => todo!("vertex settings"),
        }
    }

    fn edge_label(&self, e: EIdx) -> impl core::fmt::Display {
        match &self.flows {
            Some(flows) => flows[e].to_string(),
            None => String::new(),
        }
    }

    fn edge_tooltip(&self, e: EIdx) -> Option<impl core::fmt::Display> {
        let p = self.nw.p;
        let edge = self.graph().edge(e);
        let space = |st: SpaceTime| self.nw.p.space_key(st.space());

        Some(match edge.data() {
            ConnWaitEdge::Wait => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t = p.transport_by_idx(t);
                let [o, d] = [space(t.origin()), space(t.destination())];
                format!("Waiting edge at {o} among {o}-{d} transports")
            }
            ConnWaitEdge::Connect => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t = p.transport_by_idx(t);
                let [o, d] = [space(t.origin()), space(t.destination())];
                let [dt, at] = [t.origin().time(), t.destination().time()];
                format!("Transport edge using capacity of\n{o}-{d} at {dt}-{at}")
            }
            ConnWaitEdge::Enter => format!("Entering transport network"),
            ConnWaitEdge::Exit => {
                let t = self.graph().vertex(edge.tail()).data().get_t().expect("t");
                let t = p.transport_by_idx(t);
                let [o, d] = [space(t.origin()), space(t.destination())];
                let [dt, at] = [t.origin().time(), t.destination().time()];
                format!("Transport edge using capacity of\n{o}-{d} at {dt}-{at}")
            }
            ConnWaitEdge::Bypass(c) => {
                let com = p.commodity_by_idx(*c);
                let com_str = com_str(p, *c, &com);
                format!("Bypass edge with lost revenue cost\n{com_str}")
            }
        })
    }

    fn edge_settings(&self, e: EIdx) -> &EdgeSettings {
        match self.graph().edge(e).data() {
            ConnWaitEdge::Wait => &self.settings.wait,
            ConnWaitEdge::Connect => &self.settings.connect,
            ConnWaitEdge::Enter => &self.settings.enter,
            ConnWaitEdge::Exit => &self.settings.exit,
            ConnWaitEdge::Bypass(_) => &self.settings.bypass,
        }
    }

    fn graph(&self) -> &Self::G {
        &self.nw.g
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
