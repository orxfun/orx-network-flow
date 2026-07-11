use crate::pages::{NetworkViewIdx, PageIdx, ProblemViewIdx};
use crate::problem_variant::{Pr, PrBuilder, PrBuilderSpaces};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct AppData {
    pub page_idx: RwSignal<PageIdx>,
    pub view_idx_pr: RwSignal<ProblemViewIdx>,
    pub view_idx_nw: RwSignal<NetworkViewIdx>,
    pub spaces_basic: RwSignal<Vec<String>>,
    pub spaces_euclidean: RwSignal<Vec<(String, f64, f64)>>,
    pub spaces_geographic: RwSignal<Vec<(String, f64, f64)>>,
}

impl AppData {
    pub fn create() -> Self {
        Self {
            page_idx: RwSignal::new(Default::default()),
            view_idx_pr: RwSignal::new(Default::default()),
            view_idx_nw: RwSignal::new(Default::default()),
            spaces_basic: RwSignal::new(Default::default()),
            spaces_euclidean: RwSignal::new(Default::default()),
            spaces_geographic: RwSignal::new(Default::default()),
        }
    }

    pub fn page_keys(self) -> &'static [&'static str] {
        &PageIdx::ALL_KEYS
    }

    pub fn active_page_key(self) -> &'static str {
        self.page_idx.get().label()
    }

    pub fn set_active_page(self, page_label: &str) {
        self.page_idx.set(PageIdx::from_label(page_label))
    }

    pub fn view_keys(self) -> &'static [&'static str] {
        match self.page_idx.get() {
            PageIdx::Problem => &ProblemViewIdx::ALL_KEYS,
            PageIdx::Network => &NetworkViewIdx::ALL_KEYS,
        }
    }

    pub fn active_view_key(self) -> &'static str {
        match self.page_idx.get() {
            PageIdx::Problem => self.view_idx_pr.get().label(),
            PageIdx::Network => self.view_idx_nw.get().label(),
        }
    }

    pub fn set_active_view(self, view_label: &str) {
        match self.page_idx.get() {
            PageIdx::Problem => self.view_idx_pr.set(ProblemViewIdx::from_label(view_label)),
            PageIdx::Network => self.view_idx_nw.set(NetworkViewIdx::from_label(view_label)),
        }
    }
}
