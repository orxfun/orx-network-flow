use crate::graphs::VIdx;
use crate::graphs::core::GraphCoreBuilder;
use crate::graphs::{core::GraphCore, visualization::dot::NodeSettings};
use crate::networks::conn_nw::vertex_data::ConnNwVertex;
use crate::networks::conn_nw::visualization::dot::DotConnNw;
use crate::space_time::SpaceTime;
use crate::spaces::Space;
use crate::std_utils::{Map, Set};
use crate::time::Time;
use crate::{Problem, Variant};
use alloc::vec::Vec;

pub(super) type ConnNwGr = GraphCore<ConnNwVertex, ()>;

pub struct ConnNw<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) g: ConnNwGr,
}

impl<'a, V> ConnNw<'a, V>
where
    V: Variant,
{
    pub fn construct(p: &'a Problem<V>) -> Self {
        let g = construct_graph(p);
        Self { p, g }
    }

    pub fn as_dot_graph(
        &'a self,
        dt_ori_settings: Option<NodeSettings>,
        at_des_settings: Option<NodeSettings>,
    ) -> DotConnNw<'a, V> {
        DotConnNw::new(self, dt_ori_settings, at_des_settings)
    }

    // helpers

    pub(super) fn p(&self) -> &'a Problem<V> {
        &self.p
    }

    pub(super) fn g(&self) -> &ConnNwGr {
        &self.g
    }
}

// constructor

fn construct_graph<V: Variant>(p: &Problem<V>) -> ConnNwGr {
    let mut builder = GraphCoreBuilder::new();
    let b = &mut builder;

    let mut st_to_vidx: Map<SpaceTime, VIdx> = Map::new();
    let mut times_by_space: Map<Space, Set<Time>> = Map::new();

    for data in p.transports.values() {
        let dt_ori = data.origin();
        let tail = match st_to_vidx.get(&dt_ori) {
            Some(&v) => v,
            None => {
                let v = b.vertex(ConnNwVertex::St(dt_ori));
                st_to_vidx.insert(dt_ori, v);
                v
            }
        };
        // if !st_to_vidx.contains_key(&dt_ori) {
        //     let v = b.vertex(ConnNwVertex::St(dt_ori));
        //     st_to_vidx.insert(dt_ori, v);
        // }

        let (ori, dt) = (dt_ori.space(), dt_ori.time());
        times_by_space.entry(ori).or_default().insert(dt);

        let at_des = data.destination();
        let head = match st_to_vidx.get(&at_des) {
            Some(&v) => v,
            None => {
                let v = b.vertex(ConnNwVertex::St(at_des));
                st_to_vidx.insert(at_des, v);
                v
            }
        };

        let (des, at) = (at_des.space(), at_des.time());
        times_by_space.entry(des).or_default().insert(at);

        b.edge((), tail, head);
    }

    let sorted = |x: Set<Time>| {
        let mut vec: Vec<_> = x.into_iter().collect();
        vec.sort();
        vec
    };
    let times_by_space: Map<Space, Vec<Time>> = times_by_space
        .into_iter()
        .map(|(x, y)| (x, sorted(y)))
        .collect();

    builder.finish()
}
