use crate::app_data::AppData;
use leptos::*;

#[component]
pub fn ViewConnectionSettings() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <p>ConnectionSettings view</p>
    }
}
