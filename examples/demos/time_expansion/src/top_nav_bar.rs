use crate::app_data::AppData;
use crate::pages::PageIdx;
use leptos::*;

#[component]
pub fn TopNavBar() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <header class="top-nav">
            <div class="top-nav__inner">
                <div class="top-nav__brand">
                    <span class="top-nav__title">"time_expansion"</span>
                    <span class="top-nav__subtitle">"interactive network flow demo"</span>
                </div>
                <nav class="top-nav__links" aria-label="Primary">
                    <For
                        each=move || app.page_keys().to_vec()
                        key=|page| page.to_string()
                        children=move |page| {
                            view! {
                                <button
                                    class="top-nav__link"
                                    class=("top-nav__link--active", move || app.active_page_key() == page)
                                    on:click=move |_| app.set_active_page(page)
                                >
                                    {page}
                                </button>
                            }
                        }
                    />
                </nav>
            </div>
        </header>
    }
}
