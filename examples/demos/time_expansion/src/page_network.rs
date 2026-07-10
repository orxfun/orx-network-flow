use crate::app_data::AppData;
use leptos::*;

#[component]
pub fn PageNetwork() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <p>Network page</p>
    }
}
