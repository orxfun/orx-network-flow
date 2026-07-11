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
                <button class="left-nav__button" type="button">"Overview"</button>
                <button class="left-nav__button" type="button">"Inputs"</button>
                <button class="left-nav__button" type="button">"Results"</button>
            </div>
        </aside>
    }
}
