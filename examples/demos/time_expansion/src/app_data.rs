use crate::page::PageIdx;
use leptos::prelude::*;

#[derive(Clone)]
pub struct AppData {
    pub page_idx: RwSignal<PageIdx>,
}

impl AppData {
    pub fn create() -> Self {
        Self {
            page_idx: RwSignal::new(PageIdx::default()),
        }
    }
}
