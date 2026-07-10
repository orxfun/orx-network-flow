mod app_data;
mod page;
mod page_network;
mod page_problem;
mod top_nav_bar;

use crate::app_data::AppData;
use crate::page::PageIdx;
use crate::page_network::PageNetwork;
use crate::page_problem::PageProblem;
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
    let current_page = app_data.page_idx;

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
                        PageIdx::Problem => PageProblem().into_view(),
                        PageIdx::Network => PageNetwork().into_view(),
                    }}
                </section>
            </main>
        </div>
    }
}
