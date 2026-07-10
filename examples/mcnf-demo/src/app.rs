use crate::components::{FormProblem, NetworkSelector, StatsPanel};
use leptos::*;
use serde_json::Value;

#[component]
pub fn App() -> impl IntoView {
    let (error_message, set_error) = create_signal::<Option<String>>(None);
    let (problem_built, set_problem_built) = create_signal(false);
    let (show_stats, set_show_stats) = create_signal(false);
    let (stats_data, set_stats) = create_signal::<Option<Value>>(None);
    let (problem_input, set_problem_input) =
        create_signal::<Option<crate::serialization::ProblemInput>>(None);
    let (active_tab, set_active_tab) = create_signal("input".to_string());

    let on_problem_built = move |input: crate::serialization::ProblemInput| {
        set_problem_input.set(Some(input));
        set_problem_built.set(true);
        set_error.set(None);
        set_active_tab.set("network".to_string());
    };

    let on_error = move |msg: String| {
        set_error.set(Some(msg));
    };

    let on_stats_loaded = move |stats: Value| {
        set_stats.set(Some(stats));
        set_show_stats.set(true);
        set_active_tab.set("solution".to_string());
    };

    view! {
        <div class="app-container">
            <header class="app-header">
                <h1>"MCNF Interactive Demo"</h1>
                <p>"Multi-Commodity Network Flow — interactive problem builder and solver"</p>
            </header>

            {move || {
                error_message.get().map(|msg| {
                    view! {
                        <div class="error-banner">
                            <span class="error-icon">"⚠"</span>
                            <p>{msg}</p>
                        </div>
                    }
                })
            }}

            // ── Tab bar ──────────────────────────────────────────────────
            <nav class="tab-bar">
                <button
                    class=move || format!("tab-btn{}", if active_tab.get() == "input" { " active" } else { "" })
                    on:click=move |_| set_active_tab.set("input".to_string())
                >
                    <span class="tab-icon">"①"</span>
                    "Input"
                </button>
                <div class="tab-separator"></div>
                <button
                    class=move || format!("tab-btn{}{}", if active_tab.get() == "network" { " active" } else { "" }, if !problem_built.get() { " disabled" } else { "" })
                    disabled=move || !problem_built.get()
                    on:click=move |_| { if problem_built.get() { set_active_tab.set("network".to_string()); } }
                >
                    <span class="tab-icon">"②"</span>
                    "Network"
                    {move || problem_built.get().then(|| view! { <span class="tab-badge tab-ready">"●"</span> })}
                </button>
                <div class="tab-separator"></div>
                <button
                    class=move || format!("tab-btn{}{}", if active_tab.get() == "solution" { " active" } else { "" }, if !show_stats.get() { " disabled" } else { "" })
                    disabled=move || !show_stats.get()
                    on:click=move |_| { if show_stats.get() { set_active_tab.set("solution".to_string()); } }
                >
                    <span class="tab-icon">"③"</span>
                    "Solution"
                    {move || show_stats.get().then(|| view! { <span class="tab-badge tab-solved">"●"</span> })}
                </button>
            </nav>

            // ── Tab panels ───────────────────────────────────────────────
            <div class="tab-panels">
                // Input tab
                {move || (active_tab.get() == "input").then(|| view! {
                    <div class="tab-panel">
                        <FormProblem
                            on_built=on_problem_built
                            on_error=on_error
                        />
                    </div>
                })}

                // Network tab
                {move || (active_tab.get() == "network").then(|| view! {
                    <div class="tab-panel">
                        <NetworkSelector
                            problem_input=problem_input
                            on_stats_loaded=on_stats_loaded
                            on_error=on_error
                        />
                    </div>
                })}

                // Solution tab
                {move || (active_tab.get() == "solution" && show_stats.get()).then(|| view! {
                    <div class="tab-panel">
                        <StatsPanel stats=stats_data />
                    </div>
                })}
            </div>
        </div>
    }
}
