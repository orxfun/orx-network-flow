use leptos::*;
use serde_json::{Value, json};

#[component]
pub fn NetworkSelector(
    problem_input: ReadSignal<Option<crate::serialization::ProblemInput>>,
    on_stats_loaded: impl Fn(Value) + 'static,
    on_error: impl Fn(String) + 'static,
) -> impl IntoView {
    let (network_type, set_network_type) = create_signal("aon");
    let (grouping_strategy, set_grouping_strategy) = create_signal("dd");
    let (solver_backend, set_solver_backend) = create_signal("microlp");

    let on_solve = move |_| {
        if let Some(input) = problem_input.get() {
            let network_choice = crate::serialization::NetworkChoice {
                network_type: network_type.get().to_string(),
                grouping_strategy: grouping_strategy.get().to_string(),
                solver_backend: solver_backend.get().to_string(),
            };
            match crate::solver_handler::solve_network_from_input(&input, &network_choice) {
                Ok(response) => {
                    let stats_json = json!({
                        "network_type": network_choice.network_type,
                        "grouping_strategy": network_choice.grouping_strategy,
                        "num_variables": response.num_variables,
                        "num_constraints": response.num_constraints,
                        "num_commodities": response.num_commodities,
                        "num_spaces": response.num_spaces,
                        "num_transports": response.num_transports,
                        "objective_value": response.objective_value,
                        "status": response.status,
                        "solution_data": response.solution_data,
                        "enhanced_solution_data": response.enhanced_solution_data,
                    });
                    on_stats_loaded(stats_json);
                }
                Err(e) => on_error(format!("Solve error: {}", e)),
            }
        } else {
            on_error("No problem data available".into());
        }
    };

    view! {
        <div class="network-tab-layout">
            <div class="network-summary-card">
                <h2>"Problem Summary"</h2>
                {move || {
                    problem_input.get().map(|input| view! {
                        <div class="summary-grid">
                            <div class="summary-item">
                                <span class="summary-value">{input.spaces.len()}</span>
                                <span class="summary-label">"Spaces"</span>
                            </div>
                            <div class="summary-item">
                                <span class="summary-value">{input.commodities.len()}</span>
                                <span class="summary-label">"Commodities"</span>
                            </div>
                            <div class="summary-item">
                                <span class="summary-value">{input.transports.len()}</span>
                                <span class="summary-label">"Transports"</span>
                            </div>
                            <div class="summary-item">
                                <span class="summary-value">{input.lost_revenue_costs.len()}</span>
                                <span class="summary-label">"Cost rules"</span>
                            </div>
                        </div>
                        <div class="summary-details">
                            <div class="summary-section">
                                <h4>"Spaces"</h4>
                                <div class="chip-list">
                                    {input.spaces.iter().map(|s| view! {
                                        <span class="chip chip-space">{s.name.clone()}</span>
                                    }).collect_view()}
                                </div>
                            </div>
                            <div class="summary-section">
                                <h4>"Commodities"</h4>
                                {input.commodities.iter().map(|c| view! {
                                    <div class="summary-row">
                                        <span class="summary-row-id">{format!("C{}", c.id)}</span>
                                        <span class="summary-row-route">{format!("{} → {}", c.origin, c.destination)}</span>
                                        <span class="summary-row-qty">{format!("qty {}", c.quantity)}</span>
                                    </div>
                                }).collect_view()}
                            </div>
                            <div class="summary-section">
                                <h4>"Transports"</h4>
                                {input.transports.iter().map(|t| view! {
                                    <div class="summary-row">
                                        <span class="summary-row-id">{format!("T{}", t.id)}</span>
                                        <span class="summary-row-route">{format!("{} → {}", t.origin, t.destination)}</span>
                                        <span class="summary-row-qty">{format!("cap {}", t.capacity)}</span>
                                    </div>
                                }).collect_view()}
                            </div>
                        </div>
                    })
                }}
            </div>

            <div class="network-config-card">
                <h2>"Configure Network"</h2>

                <div class="config-section">
                    <h3>"Network Type"</h3>
                    <div class="radio-cards">
                        <label class=move || format!("radio-card{}", if network_type.get() == "aon" { " selected" } else { "" })>
                            <input type="radio" name="network_type" value="aon" checked=true on:change=move |_| set_network_type.set("aon") />
                            <strong>"AON Wait"</strong>
                            <small>"Activity-On-Node"</small>
                        </label>
                        <label class=move || format!("radio-card{}", if network_type.get() == "aoa" { " selected" } else { "" })>
                            <input type="radio" name="network_type" value="aoa" on:change=move |_| set_network_type.set("aoa") />
                            <strong>"AOA Wait"</strong>
                            <small>"Activity-On-Arc"</small>
                        </label>
                    </div>
                </div>

                <div class="config-section">
                    <h3>"Grouping Strategy"</h3>
                    <div class="radio-cards">
                        <label class=move || format!("radio-card{}", if grouping_strategy.get() == "dd" { " selected" } else { "" })>
                            <input type="radio" name="grouping" value="dd" checked=true on:change=move |_| set_grouping_strategy.set("dd") />
                            <strong>"DD"</strong>
                            <small>"Due-Destination"</small>
                        </label>
                        <label class=move || format!("radio-card{}", if grouping_strategy.get() == "ro" { " selected" } else { "" })>
                            <input type="radio" name="grouping" value="ro" on:change=move |_| set_grouping_strategy.set("ro") />
                            <strong>"RO"</strong>
                            <small>"Ready-Origin"</small>
                        </label>
                    </div>
                </div>

                <div class="config-section">
                    <h3>"Solver"</h3>
                    <div class="radio-cards solver-cards">
                        <label class=move || format!("radio-card{}", if solver_backend.get() == "microlp" { " selected" } else { "" })>
                            <input type="radio" name="solver" value="microlp" checked=true on:change=move |_| set_solver_backend.set("microlp") />
                            <strong>"MicroLP"</strong>
                            <small>"Pure Rust"</small>
                        </label>
                        <label class=move || format!("radio-card{}", if solver_backend.get() == "cplex" { " selected" } else { "" })>
                            <input type="radio" name="solver" value="cplex" on:change=move |_| set_solver_backend.set("cplex") />
                            <strong>"CPLEX"</strong>
                            <small>"External"</small>
                        </label>
                        <label class=move || format!("radio-card{}", if solver_backend.get() == "highs" { " selected" } else { "" })>
                            <input type="radio" name="solver" value="highs" on:change=move |_| set_solver_backend.set("highs") />
                            <strong>"HiGHS"</strong>
                            <small>"External"</small>
                        </label>
                    </div>
                </div>

                <button class="btn-solve" on:click=on_solve>
                    "▶ Solve Network"
                </button>
            </div>
        </div>
    }
}