use crate::app_data::AppData;
use crate::pages::{NetworkViewIdx, network::connection_settings::ViewConnectionSettings};
use leptos::*;

#[component]
pub fn PageNetwork() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        {move || match app.view_idx_nw.get() {
            NetworkViewIdx::ConnectionSettings => ViewConnectionSettings().into_view(),
        }}
    }
}
