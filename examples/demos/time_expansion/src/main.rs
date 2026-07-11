mod app_data;
mod left_nav_bar;
mod pages;
mod problem_variant;
mod top_nav_bar;

use crate::app_data::AppData;
use crate::left_nav_bar::LeftNavBar;
use crate::pages::{PageIdx, PageNetwork, PageProblem};
use crate::top_nav_bar::TopNavBar;
use leptos::*;

fn main() {
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let app = AppData::create();

    provide_context(app);

    view! {
        <div class="app-shell">
            <TopNavBar />
            <main class="app-main">
                <LeftNavBar />
                <div class="app-content">
                    <section class="hero-panel">
                        <p class="eyebrow">"orx-network-flow demo"</p>
                        <h1>{move || app.page_idx.get().label()}</h1>
                        <p class="hero-copy">{move || app.page_idx.get().description()}</p>
                    </section>
                    <section class="content-panel">
                        {move || match app.page_idx.get() {
                            PageIdx::Problem => PageProblem().into_view(),
                            PageIdx::Network => PageNetwork().into_view(),
                        }}
                    </section>
                </div>
            </main>
        </div>
    }
}
