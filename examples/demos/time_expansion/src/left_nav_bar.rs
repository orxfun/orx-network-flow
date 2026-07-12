use crate::app_data::AppData;
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
                                class=("left-nav__link--active", move || app.active_view_key() == view)
                                on:click=move |_| app.set_active_view(view)
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
