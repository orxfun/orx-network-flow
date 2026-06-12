use crate::{Problem, Variant, spaces::Space, time::Time, utils::std_utils::Map};

#[derive(derive_new::new)]
pub struct TemporalConnectivityBuilder<'a, V: Variant> {
    p: &'a Problem<V>,
    conn: &'a mut TemporalConnectivity,
}

impl<V: Variant> TemporalConnectivityBuilder<'_, V> {
    pub fn global(&mut self, min: impl Into<Time>, max: impl Into<Time>) {
        let [min, max] = [min.into(), max.into()];
        assert!(max >= min);
        self.conn.global_min_max_ct = [min, max];
    }

    pub fn local(&mut self, location: &V::S, min: impl Into<Time>, max: impl Into<Time>) {
        let space = self.p.space_idx(location).expect("unknown space");
        let [min, max] = [min.into(), max.into()];
        assert!(max >= min);
        self.conn.local_min_max_ct.insert(space, [min, max]);
    }
}

pub struct TemporalConnectivity {
    pub global_min_max_ct: [Time; 2],
    pub local_min_max_ct: Map<Space, [Time; 2]>,
}

impl Default for TemporalConnectivity {
    fn default() -> Self {
        Self {
            global_min_max_ct: [Time::zero(), Time::inf()],
            local_min_max_ct: Default::default(),
        }
    }
}

impl TemporalConnectivity {
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
                let [min_ct, max_ct] = match self.local_min_max_ct.get(&space) {
                    Some(local) => local,
                    None => &self.global_min_max_ct,
                };
                let ct = second_dt - first_at;
                ct >= *min_ct && ct <= *max_ct
            }
        }
    }
}
