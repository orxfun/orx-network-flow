use crate::page::PageIdx;
use leptos::prelude::*;

#[derive(Clone)]
pub struct AppData {
    pub view: RwSignal<PageIdx>,
}

impl AppData {
    pub fn create() -> Self {
        Self {
            view: RwSignal::new(PageIdx::default()),
        }
    }
}
