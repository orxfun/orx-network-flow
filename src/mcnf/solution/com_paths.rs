use crate::{Transport, Variant};
use alloc::vec::Vec;
use core::fmt::{Debug, Display};

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
