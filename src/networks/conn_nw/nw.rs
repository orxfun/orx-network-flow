use crate::graphs::VIdx;
use crate::graphs::core::GraphCoreBuilder;
use crate::graphs::{core::GraphCore, visualization::dot::NodeSettings};
use crate::networks::conn_nw::visualization::dot::DotConnNw;
use crate::space_time::SpaceTime;
use crate::spaces::Space;
use crate::std_utils::{Map, Set};
use crate::time::Time;
use crate::{Problem, Variant};
use alloc::vec::Vec;

type G = GraphCore<(), ()>;

pub struct ConnNw<'a, V: Variant> {
    pub(super) p: &'a Problem<V>,
    pub(super) g: G,
}

impl<'a, V> ConnNw<'a, V>
where
    V: Variant,
{
    pub fn construct(p: &'a Problem<V>) -> Self {
        let g = construct_graph(p);
        Self { p, g }
    }

    pub fn as_dot_graph(&'a self, transport_settings: Option<NodeSettings>) -> DotConnNw<'a, V> {
        DotConnNw::new(self, transport_settings)
    }

    // helpers

    pub(super) fn p(&self) -> &'a Problem<V> {
        &self.p
    }

    pub(super) fn g(&self) -> &G {
        &self.g
    }
}

// constructor

fn construct_graph<V: Variant>(p: &Problem<V>) -> G {
    let mut builder = GraphCoreBuilder::new();
    let mut b = &mut builder;
    let mut v = 0;

    let mut ori_dt_to_vidx: Map<SpaceTime, VIdx> = Map::new();
    let mut dt_list_by_ori: Map<Space, Set<Time>> = Map::new();

    let mut des_at_to_vidx: Map<SpaceTime, VIdx> = Map::new();
    let mut at_list_by_des: Map<Space, Set<Time>> = Map::new();

    // // let mut ori_st: IdxMap<_, _, usize> = Default::default();
    // let mut space_rt_due: Map<Space, (Set<Time>, Set<Time>)> = Map::new();
    for data in p.transports.values() {
        let ori_dt = data.origin();
        if !ori_dt_to_vidx.contains_key(&ori_dt) {
            let v = b.vertex(());
            ori_dt_to_vidx.insert(ori_dt, v);
        }

        let (ori, dt) = (ori_dt.space(), ori_dt.time());
        dt_list_by_ori.entry(ori).or_default().insert(dt);

        let des_at = data.destination();
        if !des_at_to_vidx.contains_key(&des_at) {
            let v = b.vertex(());
            des_at_to_vidx.insert(des_at, v);
        }

        let (des, at) = (des_at.space(), des_at.time());
        at_list_by_des.entry(des).or_default().insert(at);
    }

    let sorted = |x: Set<Time>| {
        let mut v: Vec<_> = x.into_iter().collect();
        v.sort();
        v
    };
    let dt_list_by_ori: Map<Space, Vec<Time>> = dt_list_by_ori
        .into_iter()
        .map(|(x, y)| (x, sorted(y)))
        .collect();
    let at_list_by_des: Map<Space, Vec<Time>> = at_list_by_des
        .into_iter()
        .map(|(x, y)| (x, sorted(y)))
        .collect();

    // let mut v = VIdx::from(0);

    // let sorted_time = |times: Set<Time>| {
    //     let mut times: Vec<_> = times.into_iter().map(|x| (x, 0)).collect();
    //     times.sort();

    //     times
    // };

    // // let space_rt_due: Map<Space, (Vec<Time>, Vec<Time>)> = space_rt_due
    //     .into_iter()
    //     .map(|(s, (ori, des))| (s, (sorted_time(ori), sorted_time(des))))
    //     .collect();

    builder.finish()
}
