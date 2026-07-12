use crate::components::{InputCell, InputRow, InputTable, RemoveItemFn};
use leptos::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SpaceInput {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for SpaceInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

#[component]
pub fn FormSpaces(
    spaces: ReadSignal<Vec<SpaceInput>>,
    set_spaces: WriteSignal<Vec<SpaceInput>>,
) -> impl IntoView {
    view! {
        <div>12</div>
    }
}
