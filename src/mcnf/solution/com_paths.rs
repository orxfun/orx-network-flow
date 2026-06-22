use crate::{Problem, Transport, Variant};
use alloc::{string::String, vec::Vec};
use core::fmt::{Debug, Display, Write};
use orx_iterable::Iterable;

#[derive(Default)]
pub struct CommodityPaths<V: Variant> {
    pub path_flows: Vec<PathFlow<V>>,
}

impl<'a, V: Variant> IntoIterator for &'a CommodityPaths<V> {
    type Item = &'a PathFlow<V>;

    type IntoIter = core::slice::Iter<'a, PathFlow<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.path_flows.iter()
    }
}

pub struct PathFlow<V: Variant> {
    pub path: Path,
    pub flow: V::F,
}

pub enum Path {
    OneLeg([Transport; 1]),
    TwoLegs([Transport; 2]),
    ThreeLegs([Transport; 3]),
    Long(Vec<Transport>),
}

impl Path {
    pub fn drain_from(transports: &mut Vec<Transport>) -> Self {
        Self::collect(transports.drain(..))
    }

    pub fn collect<T>(transports: T) -> Self
    where
        T: IntoIterator<Item = Transport>,
        T::IntoIter: ExactSizeIterator,
    {
        let mut transports = transports.into_iter();
        match transports.len() {
            1 => {
                let i = transports.next().expect("len=1");
                Self::OneLeg([i])
            }
            2 => {
                let i = transports.next().expect("len=2");
                let j = transports.next().expect("len=2");
                Self::TwoLegs([i, j])
            }
            3 => {
                let i = transports.next().expect("len=3");
                let j = transports.next().expect("len=3");
                let k = transports.next().expect("len=3");
                Self::ThreeLegs([i, j, k])
            }
            _ => Self::Long(transports.collect()),
        }
    }

    pub fn nth(&self, n: usize) -> Option<Transport> {
        match (self, n) {
            (Self::OneLeg([t]), 0) => Some(*t),
            (Self::TwoLegs([t, _]), 0) => Some(*t),
            (Self::TwoLegs([_, t]), 1) => Some(*t),
            (Self::ThreeLegs([t, _, _]), 0) => Some(*t),
            (Self::ThreeLegs([_, t, _]), 1) => Some(*t),
            (Self::ThreeLegs([_, _, t]), 2) => Some(*t),
            (Self::Long(v), _) => v.get(n).copied(),
            _ => None,
        }
    }

    pub fn first(&self) -> Option<Transport> {
        self.nth(0)
    }

    pub fn last(&self) -> Option<Transport> {
        let n = match self {
            Self::OneLeg(_) => 0,
            Self::TwoLegs(_) => 1,
            Self::ThreeLegs(_) => 2,
            Self::Long(v) => v.len() - 1,
        };
        self.nth(n)
    }

    pub fn as_slice(&self) -> &[Transport] {
        match self {
            Path::OneLeg(x) => x,
            Path::TwoLegs(x) => x,
            Path::ThreeLegs(x) => x,
            Path::Long(x) => x,
        }
    }

    pub fn used_transports<V: Variant>(&self, p: &Problem<V>) -> impl Iterator<Item = Transport> {
        let is_transport = |(i, t): &(usize, Transport)| match self.as_slice().get(i + 1) {
            Some(&next) => {
                let ori1 = p.transport_by_idx(*t).origin().space();
                let ori2 = p.transport_by_idx(next).origin().space();
                ori1 != ori2
            }
            None => true,
        };
        self.iter().enumerate().filter(is_transport).map(|x| x.1)
    }

    pub fn to_str_as_spaces<V: Variant>(&self, p: &Problem<V>) -> String {
        let mut str = String::new();
        let mut started = false;

        for t in self.used_transports(p) {
            let t = p.transport_by_idx(t);
            match started {
                false => {
                    write!(&mut str, "{}", p.space_key(t.origin().space())).expect("build-str");
                    write!(&mut str, "-{}", p.space_key(t.destination().space()))
                        .expect("build-str");
                }
                true => write!(&mut str, "-{}", p.space_key(t.destination().space()))
                    .expect("build-str"),
            }
            started = true;
        }

        str
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = Transport;

    type IntoIter = core::iter::Copied<core::slice::Iter<'a, Transport>>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter().copied()
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut started = false;
        for x in self.into_iter() {
            match started {
                true => write!(f, "-{x}")?,
                false => write!(f, "{x}")?,
            }
            started = true;
        }
        Ok(())
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
