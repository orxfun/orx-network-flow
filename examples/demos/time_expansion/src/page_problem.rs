use crate::app_data::AppData;
use leptos::*;

#[component]
pub fn PageProblem() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <p>Problem page</p>
    }
}
