use crate::app_data::AppData;
use leptos::*;

#[component]
pub fn ViewCommodities() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <p>Commodities view</p>
    }
}
