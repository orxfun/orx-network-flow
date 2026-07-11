mod app_data;
mod left_nav_bar;
mod pages;
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
    let app_data = AppData::create();
    let current_page = app_data.page_idx;

    provide_context(app_data);

    view! {
        <div class="app-shell">
            <TopNavBar />
            <main class="app-main">
                <LeftNavBar />
                <div class="app-content">
                    <section class="hero-panel">
                        <p class="eyebrow">"orx-network-flow demo"</p>
                        <h1>{move || current_page.get().label()}</h1>
                        <p class="hero-copy">{move || current_page.get().description()}</p>
                    </section>
                    <section class="content-panel">
                        {move || match current_page.get() {
                            PageIdx::Problem(_) => PageProblem().into_view(),
                            PageIdx::Network(_) => PageNetwork().into_view(),
                        }}
                    </section>
                </div>
            </main>
        </div>
    }
}
