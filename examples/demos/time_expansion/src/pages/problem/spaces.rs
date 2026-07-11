use crate::app_data::AppData;
use leptos::*;

pub enum SpaceKind {
    Basic,
    Euclidean,
    Geographic,
}

#[component]
pub fn ViewSpaces() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <p>Spaces view</p>
    }
}
