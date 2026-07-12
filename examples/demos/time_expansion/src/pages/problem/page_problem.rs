use crate::pages::problem::{
    commodities::ViewCommodities, spaces::ViewSpaces, transports::ViewTransports,
};
use crate::{app_data::AppData, pages::ProblemViewIdx};
use leptos::*;

#[component]
pub fn PageProblem() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        {move || match app.view_idx_pr.get() {
            ProblemViewIdx::Spaces => ViewSpaces().into_view(),
            ProblemViewIdx::Commodities => ViewCommodities().into_view(),
            ProblemViewIdx::Transports => ViewTransports().into_view(),
        }}
    }
}
