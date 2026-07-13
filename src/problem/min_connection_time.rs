use crate::spaces::Spaces;
use crate::{Problem, Variant, spaces::Space, time::Time, utils::std_utils::Map};

#[derive(derive_new::new)]
pub struct MinConnectionTimeBuilder<'a, V: Variant> {
    spaces: &'a Spaces<V>,
    conn: &'a mut MinConnectionTime,
}

impl<V: Variant> MinConnectionTimeBuilder<'_, V> {
    pub fn global(&mut self, min: impl Into<Time>) {
        self.conn.global_min_ct = min.into();
    }

    pub fn local(&mut self, location: &V::S, min: impl Into<Time>) {
        let space = self.spaces.get_ind_by_key(location).expect("unknown space");
        self.conn.local_min_ct.insert(space, min.into());
    }
}

pub struct MinConnectionTime {
    pub global_min_ct: Time,
    pub local_min_ct: Map<Space, Time>,
}

impl Default for MinConnectionTime {
    fn default() -> Self {
        Self {
            global_min_ct: Time::zero(),
            local_min_ct: Default::default(),
        }
    }
}

impl MinConnectionTime {
    pub fn can_connect<V: Variant>(
        &self,
        p: &Problem<V>,
        space: Space,
        first_at: Time,
        second_dt: Time,
    ) -> bool {
        match second_dt >= first_at {
            false => false,
            true => {
                let min_ct = match self.local_min_ct.get(&space) {
                    Some(local) => local,
                    None => &self.global_min_ct,
                };
                let ct = second_dt - first_at;
                ct >= *min_ct
            }
        }
    }
}
