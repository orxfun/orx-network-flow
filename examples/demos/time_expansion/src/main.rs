mod app_data;
mod page;
mod top_nav_bar;

use crate::app_data::AppData;
use crate::page::PageIdx;
use crate::top_nav_bar::TopNavBar;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    })
}

#[component]
fn App() -> impl IntoView {
    let app_data = AppData::create();
    let current_page = app_data.view;

    provide_context(app_data);

    view! {
        <div class="app-shell">
            <TopNavBar />
            <main class="app-main">
                <section class="hero-panel">
                    <p class="eyebrow">"orx-network-flow demo"</p>
                    <h1>{move || current_page.get().label()}</h1>
                    <p class="hero-copy">{move || current_page.get().description()}</p>
                </section>
                <section class="content-panel">
                    {move || match current_page.get() {
                        PageIdx::Problem => view! {
                            <div class="placeholder-copy">
                                "This page can host problem parameters, time periods, and scenario controls."
                            </div>
                        }
                            .into_view(),
                        PageIdx::Network => view! {
                            <div class="placeholder-copy">
                                "This page can host graph visualizations, node details, and flow tables."
                            </div>
                        }
                            .into_view(),
                    }}
                </section>
            </main>
        </div>
    }
}
