use leptos::*;

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>"time_expansion"</div>
    }
}
