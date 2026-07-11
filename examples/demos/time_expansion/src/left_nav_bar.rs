use crate::app_data::AppData;
use crate::pages::PageIdx;
use leptos::*;

#[component]
pub fn LeftNavBar() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <aside class="left-nav" aria-label="Within page navigation">
            <div class="left-nav__inner">
                <p class="left-nav__title">"Within This Page"</p>
                <For
                    each=move || app.view_keys().to_vec()
                    key=|view| view.to_string()
                    children=move |view| {
                        view! {
                            <button
                                class="left-nav__button"
                            >
                                {view}
                            </button>
                        }
                    }
                />
            </div>
        </aside>
    }
}
