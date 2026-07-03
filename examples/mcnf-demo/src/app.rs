use leptos::*;
use serde_json::{Value, json};

#[component]
pub fn App() -> impl IntoView {
    let (error_message, set_error) = create_signal::<Option<String>>(None);
    let (problem_built, set_problem_built) = create_signal(false);
    let (show_stats, set_show_stats) = create_signal(false);
    let (stats_data, set_stats) = create_signal::<Option<Value>>(None);

    let on_problem_built = move |_| {
        set_problem_built.set(true);
        set_error.set(None);
    };

    let on_error = move |msg: String| {
        set_error.set(Some(msg));
        set_problem_built.set(false);
    };

    let on_stats_loaded = move |stats: Value| {
        set_stats.set(Some(stats));
        set_show_stats.set(true);
    };

    view! {
        <div class="app-container">
            <header class="app-header">
                <h1>"MCNF Interactive Demo"</h1>
                <p>"Solve network flow problems with configurable solvers"</p>
            </header>

            {move || {
                error_message.get().map(|msg| {
                    view! {
                        <div class="error-banner">
                            <p>{msg}</p>
                        </div>
                    }
                })
            }}

            <div class="content-grid">
                <section class="form-section">
                    <ProblemForm
                        on_built=on_problem_built
                        on_error=on_error
                    />
                </section>

                {move || {
                    if problem_built.get() {
                        view! {
                            <section class="selector-section">
                                <NetworkSelector
                                    on_stats_loaded=on_stats_loaded
                                    on_error=on_error
                                />
                            </section>
                        }
                    } else {
                        view! {
                            <section class="selector-section disabled">
                                <p>"Complete the problem form to enable"</p>
                            </section>
                        }
                    }
                }}

                {move || {
                    if show_stats.get() {
                        view! {
                            <section class="stats-section">
                                <StatsPanel stats=stats_data />
                            </section>
                        }
                    } else {
                        view! {
                            <section class="stats-section placeholder">
                                <p>"Statistics will appear here"</p>
                            </section>
                        }
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn ProblemForm(
    on_built: impl Fn(()) + 'static,
    on_error: impl Fn(String) + 'static,
) -> impl IntoView {
    let (spaces, set_spaces) = create_signal(vec![SpaceInput::default()]);
    let (commodities, set_commodities) = create_signal(vec![]);
    let (transports, set_transports) = create_signal(vec![]);

    let add_space = move |_| {
        set_spaces.update(|s| s.push(SpaceInput::default()));
    };

    let remove_space = move |idx| {
        set_spaces.update(|s| {
            if s.len() > 1 {
                s.remove(idx);
            }
        });
    };

    let add_commodity = move |_| {
        set_commodities.update(|c| c.push(CommodityInput::default()));
    };

    let remove_commodity = move |idx| {
        set_commodities.update(|c| {
            if c.len() > idx {
                c.remove(idx);
            }
        });
    };

    let add_transport = move |_| {
        set_transports.update(|t| t.push(TransportInput::default()));
    };

    let remove_transport = move |idx| {
        set_transports.update(|t| {
            if t.len() > idx {
                t.remove(idx);
            }
        });
    };

    let on_submit = move |_| {
        if spaces.get().is_empty() {
            on_error("At least one space is required".into());
            return;
        }

        // Here you would call the backend command
        // For now, just trigger the event
        on_built(());
    };

    view! {
        <div class="problem-form">
            <h2>"Define Problem"</h2>

            <fieldset>
                <legend>"Geographic Spaces"</legend>
                <div class="form-group">
                    {move || {
                        spaces
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(idx, _)| {
                                view! {
                                    <div class="form-row">
                                        <input type="text" placeholder="Space name" />
                                        <input type="number" placeholder="Latitude" step="0.001" />
                                        <input type="number" placeholder="Longitude" step="0.001" />
                                        {move || {
                                            (spaces.get().len() > 1).then(|| {
                                                view! {
                                                    <button
                                                        class="btn-remove"
                                                        on:click=move |_| remove_space(idx)
                                                    >
                                                        "Remove"
                                                    </button>
                                                }
                                            })
                                        }}
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                    <button class="btn-add" on:click=add_space>
                        "+ Add Space"
                    </button>
                </div>
            </fieldset>

            <fieldset>
                <legend>"Commodities"</legend>
                <div class="form-group">
                    {move || {
                        commodities
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(idx, _)| {
                                view! {
                                    <div class="form-row">
                                        <input type="number" placeholder="Commodity ID" />
                                        <input type="text" placeholder="Origin" />
                                        <input type="number" placeholder="Ready time" />
                                        <input type="text" placeholder="Destination" />
                                        <input type="number" placeholder="Due time" />
                                        <input type="number" placeholder="Quantity" />
                                        <button
                                            class="btn-remove"
                                            on:click=move |_| remove_commodity(idx)
                                        >
                                            "Remove"
                                        </button>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                    <button class="btn-add" on:click=add_commodity>
                        "+ Add Commodity"
                    </button>
                </div>
            </fieldset>

            <fieldset>
                <legend>"Transports"</legend>
                <div class="form-group">
                    {move || {
                        transports
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(idx, _)| {
                                view! {
                                    <div class="form-row">
                                        <input type="number" placeholder="Transport ID" />
                                        <input type="text" placeholder="Vehicle type" />
                                        <input type="text" placeholder="Origin" />
                                        <input type="number" placeholder="Departure time" />
                                        <input type="text" placeholder="Destination" />
                                        <input type="number" placeholder="Arrival time" />
                                        <input type="number" placeholder="Capacity" />
                                        <button
                                            class="btn-remove"
                                            on:click=move |_| remove_transport(idx)
                                        >
                                            "Remove"
                                        </button>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                    <button class="btn-add" on:click=add_transport>
                        "+ Add Transport"
                    </button>
                </div>
            </fieldset>

            <button class="btn-submit" on:click=on_submit>
                "Build Problem"
            </button>
        </div>
    }
}

#[component]
fn NetworkSelector(
    on_stats_loaded: impl Fn(Value) + 'static,
    on_error: impl Fn(String) + 'static,
) -> impl IntoView {
    let (network_type, set_network_type) = create_signal("aon");
    let (grouping_strategy, set_grouping_strategy) = create_signal("dd");
    let (solver_backend, set_solver_backend) = create_signal("microlp");

    let on_solve = move |_| {
        // Mock stats response
        let stats = json!({
            "num_variables": 100,
            "num_constraints": 50,
            "num_commodities": 1,
            "num_spaces": 1,
            "num_transports": 0,
        });
        on_stats_loaded(stats);
    };

    view! {
        <div class="network-selector">
            <h2>"Configure Network"</h2>

            <fieldset>
                <legend>"Network Type"</legend>
                <label>
                    <input
                        type="radio"
                        name="network_type"
                        value="aon"
                        checked=true
                        on:change=move |_| set_network_type.set("aon")
                    />
                    "Activity-On-Node (AON) Wait"
                </label>
                <label>
                    <input
                        type="radio"
                        name="network_type"
                        value="aoa"
                        on:change=move |_| set_network_type.set("aoa")
                    />
                    "Activity-On-Arc (AOA) Wait"
                </label>
            </fieldset>

            <fieldset>
                <legend>"Grouping Strategy"</legend>
                <label>
                    <input
                        type="radio"
                        name="grouping"
                        value="dd"
                        checked=true
                        on:change=move |_| set_grouping_strategy.set("dd")
                    />
                    "Demand-Demand (DD)"
                </label>
                <label>
                    <input
                        type="radio"
                        name="grouping"
                        value="ro"
                        on:change=move |_| set_grouping_strategy.set("ro")
                    />
                    "Reception-Order (RO)"
                </label>
            </fieldset>

            <fieldset>
                <legend>"Solver Backend"</legend>
                <label>
                    <input
                        type="radio"
                        name="solver"
                        value="microlp"
                        checked=true
                        on:change=move |_| set_solver_backend.set("microlp")
                    />
                    "MicroLP (Pure Rust)"
                </label>
                <label>
                    <input
                        type="radio"
                        name="solver"
                        value="cplex"
                        on:change=move |_| set_solver_backend.set("cplex")
                    />
                    "CPLEX (External)"
                </label>
                <label>
                    <input
                        type="radio"
                        name="solver"
                        value="highs"
                        on:change=move |_| set_solver_backend.set("highs")
                    />
                    "HiGHS"
                </label>
                <label>
                    <input
                        type="radio"
                        name="solver"
                        value="scip"
                        on:change=move |_| set_solver_backend.set("scip")
                    />
                    "SCIP"
                </label>
                <label>
                    <input
                        type="radio"
                        name="solver"
                        value="cbc"
                        on:change=move |_| set_solver_backend.set("cbc")
                    />
                    "CBC"
                </label>
            </fieldset>

            <button class="btn-solve" on:click=on_solve>
                "Solve Network"
            </button>
        </div>
    }
}

#[component]
fn StatsPanel(stats: ReadSignal<Option<Value>>) -> impl IntoView {
    view! {
        <div class="stats-panel">
            <h2>"Network Statistics"</h2>

            {move || {
                stats.get().map(|s| {
                    view! {
                        <div class="stats-grid">
                            <div class="stat-card">
                                <div class="stat-value">
                                    {s.get("num_variables").and_then(|v| v.as_i64()).unwrap_or(0)}
                                </div>
                                <div class="stat-label">"Variables"</div>
                            </div>

                            <div class="stat-card">
                                <div class="stat-value">
                                    {s.get("num_constraints").and_then(|v| v.as_i64()).unwrap_or(0)}
                                </div>
                                <div class="stat-label">"Constraints"</div>
                            </div>

                            <div class="stat-card">
                                <div class="stat-value">
                                    {s.get("num_commodities").and_then(|v| v.as_i64()).unwrap_or(0)}
                                </div>
                                <div class="stat-label">"Commodities"</div>
                            </div>

                            <div class="stat-card">
                                <div class="stat-value">
                                    {s.get("num_spaces").and_then(|v| v.as_i64()).unwrap_or(0)}
                                </div>
                                <div class="stat-label">"Spaces"</div>
                            </div>

                            <div class="stat-card">
                                <div class="stat-value">
                                    {s.get("num_transports").and_then(|v| v.as_i64()).unwrap_or(0)}
                                </div>
                                <div class="stat-label">"Transports"</div>
                            </div>
                        </div>
                    }
                })
            }}
        </div>
    }
}

#[derive(Clone, Debug)]
struct SpaceInput {
    name: String,
    latitude: f64,
    longitude: f64,
}

impl Default for SpaceInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct CommodityInput {
    id: usize,
    origin: String,
    ready_time: i64,
    destination: String,
    due_time: i64,
    quantity: u64,
}

impl Default for CommodityInput {
    fn default() -> Self {
        Self {
            id: 0,
            origin: String::new(),
            ready_time: 0,
            destination: String::new(),
            due_time: 0,
            quantity: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct TransportInput {
    id: usize,
    vehicle_type: String,
    origin: String,
    departure_time: i64,
    destination: String,
    arrival_time: i64,
    capacity: u64,
}

impl Default for TransportInput {
    fn default() -> Self {
        Self {
            id: 0,
            vehicle_type: String::new(),
            origin: String::new(),
            departure_time: 0,
            destination: String::new(),
            arrival_time: 0,
            capacity: 0,
        }
    }
}
