use crate::app_data::AppData;
use leptos::*;

#[component]
pub fn ViewTransports() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <p>Transports view</p>
    }
}
