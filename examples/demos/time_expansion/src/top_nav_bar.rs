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
                        each=move || PageIdx::ALL
                        key=|page| page.key()
                        children=move |page| {
                            let is_active = move || app.page_idx.get() == page;

                            view! {
                                <button
                                    class="top-nav__link"
                                    class=("top-nav__link--active", is_active)
                                    on:click=move |_| app.page_idx.set(page)
                                >
                                    {page.label()}
                                </button>
                            }
                        }
                    />
                </nav>
            </div>
        </header>
    }
}
