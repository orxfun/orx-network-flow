mod app_data;
mod page;

use crate::app_data::AppData;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    })
}

#[component]
fn App() -> impl IntoView {
    provide_context(AppData::create());

    view! {
        <div>"time_expansion"</div>
    }
}
