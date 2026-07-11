use crate::pages::{NetworkViewIdx, PageIdx, ProblemViewIdx};
use leptos::prelude::*;

#[derive(Clone)]
pub struct AppData {
    pub page_idx: RwSignal<PageIdx>,
    pub view_idx_pr: RwSignal<ProblemViewIdx>,
    pub view_idx_nw: RwSignal<NetworkViewIdx>,
}

impl AppData {
    pub fn create() -> Self {
        Self {
            page_idx: RwSignal::new(Default::default()),
            view_idx_pr: RwSignal::new(Default::default()),
            view_idx_nw: RwSignal::new(Default::default()),
        }
    }

    pub fn view_keys(&self) -> &'static [&'static str] {
        match self.page_idx.get() {
            PageIdx::Problem => &ProblemViewIdx::ALL_KEYS,
            PageIdx::Network => &NetworkViewIdx::ALL_KEYS,
        }
    }

    // pub fn active_view_key(&self) -> &'static str {
    //     match self.page_idx.get() {
    //         PageIdx::Problem => &ProblemViewIdx::ALL_KEYS,
    //         PageIdx::Network => &NetworkViewIdx::ALL_KEYS,
    //     }
    // }
}
