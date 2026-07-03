use leptos::*;
use serde_json::{Value, json};

/// Generate demo data from shared_problem.rs
fn demo_data() -> (Vec<SpaceInput>, Vec<CommodityInput>, Vec<TransportInput>) {
    let spaces = vec![
        SpaceInput {
            name: "AMS".to_string(),
            latitude: 52.308_613,
            longitude: 4.763_889,
        },
        SpaceInput {
            name: "BRU".to_string(),
            latitude: 50.901_389,
            longitude: 4.484_444,
        },
        SpaceInput {
            name: "LEJ".to_string(),
            latitude: 51.25,
            longitude: 12.14,
        },
        SpaceInput {
            name: "CVG".to_string(),
            latitude: 39.0488,
            longitude: -84.6678,
        },
        SpaceInput {
            name: "SIN".to_string(),
            latitude: 1.350_189,
            longitude: 103.994_433,
        },
        SpaceInput {
            name: "EMA".to_string(),
            latitude: 52.831_111,
            longitude: -1.328_056,
        },
    ];

    let commodities = vec![
        CommodityInput {
            id: 0,
            origin: "AMS".to_string(),
            ready_time: 0,
            destination: "BRU".to_string(),
            due_time: 20,
            quantity: 100,
        },
        CommodityInput {
            id: 1,
            origin: "AMS".to_string(),
            ready_time: 0,
            destination: "CVG".to_string(),
            due_time: 20,
            quantity: 100,
        },
        CommodityInput {
            id: 2,
            origin: "AMS".to_string(),
            ready_time: 0,
            destination: "LEJ".to_string(),
            due_time: 20,
            quantity: 100,
        },
        CommodityInput {
            id: 3,
            origin: "AMS".to_string(),
            ready_time: 0,
            destination: "LEJ".to_string(),
            due_time: 20,
            quantity: 100,
        },
        CommodityInput {
            id: 4,
            origin: "LEJ".to_string(),
            ready_time: 0,
            destination: "CVG".to_string(),
            due_time: 20,
            quantity: 100,
        },
    ];

    let transports = vec![
        TransportInput {
            id: 0,
            vehicle_type: "77X".to_string(),
            origin: "AMS".to_string(),
            departure_time: 1,
            destination: "BRU".to_string(),
            arrival_time: 2,
            capacity: 10,
        },
        TransportInput {
            id: 1,
            vehicle_type: "77X".to_string(),
            origin: "AMS".to_string(),
            departure_time: 4,
            destination: "BRU".to_string(),
            arrival_time: 5,
            capacity: 10,
        },
        TransportInput {
            id: 2,
            vehicle_type: "77X".to_string(),
            origin: "AMS".to_string(),
            departure_time: 4,
            destination: "LEJ".to_string(),
            arrival_time: 5,
            capacity: 10,
        },
        TransportInput {
            id: 3,
            vehicle_type: "77X".to_string(),
            origin: "LEJ".to_string(),
            departure_time: 1,
            destination: "BRU".to_string(),
            arrival_time: 2,
            capacity: 10,
        },
        TransportInput {
            id: 4,
            vehicle_type: "77X".to_string(),
            origin: "LEJ".to_string(),
            departure_time: 4,
            destination: "BRU".to_string(),
            arrival_time: 5,
            capacity: 10,
        },
        TransportInput {
            id: 5,
            vehicle_type: "77X".to_string(),
            origin: "BRU".to_string(),
            departure_time: 7,
            destination: "CVG".to_string(),
            arrival_time: 12,
            capacity: 10,
        },
    ];

    (spaces, commodities, transports)
}

#[component]
pub fn App() -> impl IntoView {
    let (error_message, set_error) = create_signal::<Option<String>>(None);
    let (problem_built, set_problem_built) = create_signal(false);
    let (show_stats, set_show_stats) = create_signal(false);
    let (stats_data, set_stats) = create_signal::<Option<Value>>(None);
    let (problem_input, set_problem_input) =
        create_signal::<Option<crate::serialization::ProblemInput>>(None);

    let on_problem_built = move |input: crate::serialization::ProblemInput| {
        set_problem_input.set(Some(input));
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
                                    problem_input=problem_input
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
    on_built: impl Fn(crate::serialization::ProblemInput) + 'static,
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

        // Convert form inputs to serialization types
        let problem_input = crate::serialization::ProblemInput {
            spaces: spaces
                .get()
                .into_iter()
                .map(|s| crate::serialization::FormGeographicSpace {
                    name: s.name,
                    latitude: s.latitude,
                    longitude: s.longitude,
                })
                .collect(),
            commodities: commodities
                .get()
                .into_iter()
                .map(|c| crate::serialization::FormCommodity {
                    id: c.id,
                    origin: c.origin,
                    ready_time: c.ready_time,
                    destination: c.destination,
                    due_time: c.due_time,
                    quantity: c.quantity,
                })
                .collect(),
            transports: transports
                .get()
                .into_iter()
                .map(|t| crate::serialization::FormTransport {
                    id: t.id,
                    vehicle_type: t.vehicle_type,
                    origin: t.origin,
                    departure_time: t.departure_time,
                    destination: t.destination,
                    arrival_time: t.arrival_time,
                    capacity: t.capacity,
                })
                .collect(),
            lost_revenue_costs: vec![], // TODO: Add lost revenue UI
        };

        on_built(problem_input);
    };

    let load_demo_input = move |_| {
        let (demo_spaces, demo_commodities, demo_transports) = demo_data();
        set_spaces.set(demo_spaces);
        set_commodities.set(demo_commodities);
        set_transports.set(demo_transports);
    };

    view! {
        <div class="problem-form">
            <div class="form-header">
                <h2>"Define Problem"</h2>
                <button class="btn-demo" on:click=load_demo_input>
                    "📋 Demo Input"
                </button>
            </div>

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
                                        <input
                                            type="text"
                                            placeholder="Space name"
                                            prop:value=move || spaces.get().get(idx).map(|s| s.name.clone()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_spaces.update(|s| {
                                                    if let Some(space) = s.get_mut(idx) {
                                                        space.name = event_target_value(&e);
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Latitude"
                                            step="0.001"
                                            prop:value=move || spaces.get().get(idx).map(|s| s.latitude.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_spaces.update(|s| {
                                                    if let Some(space) = s.get_mut(idx) {
                                                        if let Ok(lat) = event_target_value(&e).parse::<f64>() {
                                                            space.latitude = lat;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Longitude"
                                            step="0.001"
                                            prop:value=move || spaces.get().get(idx).map(|s| s.longitude.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_spaces.update(|s| {
                                                    if let Some(space) = s.get_mut(idx) {
                                                        if let Ok(lon) = event_target_value(&e).parse::<f64>() {
                                                            space.longitude = lon;
                                                        }
                                                    }
                                                });
                                            }
                                        />
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
                                        <input
                                            type="number"
                                            placeholder="Commodity ID"
                                            prop:value=move || commodities.get().get(idx).map(|c| c.id.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_commodities.update(|c| {
                                                    if let Some(commodity) = c.get_mut(idx) {
                                                        if let Ok(id) = event_target_value(&e).parse::<usize>() {
                                                            commodity.id = id;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="text"
                                            placeholder="Origin"
                                            prop:value=move || commodities.get().get(idx).map(|c| c.origin.clone()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_commodities.update(|c| {
                                                    if let Some(commodity) = c.get_mut(idx) {
                                                        commodity.origin = event_target_value(&e);
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Ready time"
                                            prop:value=move || commodities.get().get(idx).map(|c| c.ready_time.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_commodities.update(|c| {
                                                    if let Some(commodity) = c.get_mut(idx) {
                                                        if let Ok(rt) = event_target_value(&e).parse::<i64>() {
                                                            commodity.ready_time = rt;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="text"
                                            placeholder="Destination"
                                            prop:value=move || commodities.get().get(idx).map(|c| c.destination.clone()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_commodities.update(|c| {
                                                    if let Some(commodity) = c.get_mut(idx) {
                                                        commodity.destination = event_target_value(&e);
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Due time"
                                            prop:value=move || commodities.get().get(idx).map(|c| c.due_time.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_commodities.update(|c| {
                                                    if let Some(commodity) = c.get_mut(idx) {
                                                        if let Ok(due) = event_target_value(&e).parse::<i64>() {
                                                            commodity.due_time = due;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Quantity"
                                            prop:value=move || commodities.get().get(idx).map(|c| c.quantity.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_commodities.update(|c| {
                                                    if let Some(commodity) = c.get_mut(idx) {
                                                        if let Ok(qty) = event_target_value(&e).parse::<u64>() {
                                                            commodity.quantity = qty;
                                                        }
                                                    }
                                                });
                                            }
                                        />
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
                                        <input
                                            type="number"
                                            placeholder="Transport ID"
                                            prop:value=move || transports.get().get(idx).map(|t| t.id.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_transports.update(|t| {
                                                    if let Some(transport) = t.get_mut(idx) {
                                                        if let Ok(id) = event_target_value(&e).parse::<usize>() {
                                                            transport.id = id;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="text"
                                            placeholder="Vehicle type"
                                            prop:value=move || transports.get().get(idx).map(|t| t.vehicle_type.clone()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_transports.update(|t| {
                                                    if let Some(transport) = t.get_mut(idx) {
                                                        transport.vehicle_type = event_target_value(&e);
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="text"
                                            placeholder="Origin"
                                            prop:value=move || transports.get().get(idx).map(|t| t.origin.clone()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_transports.update(|t| {
                                                    if let Some(transport) = t.get_mut(idx) {
                                                        transport.origin = event_target_value(&e);
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Departure time"
                                            prop:value=move || transports.get().get(idx).map(|t| t.departure_time.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_transports.update(|t| {
                                                    if let Some(transport) = t.get_mut(idx) {
                                                        if let Ok(dt) = event_target_value(&e).parse::<i64>() {
                                                            transport.departure_time = dt;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="text"
                                            placeholder="Destination"
                                            prop:value=move || transports.get().get(idx).map(|t| t.destination.clone()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_transports.update(|t| {
                                                    if let Some(transport) = t.get_mut(idx) {
                                                        transport.destination = event_target_value(&e);
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Arrival time"
                                            prop:value=move || transports.get().get(idx).map(|t| t.arrival_time.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_transports.update(|t| {
                                                    if let Some(transport) = t.get_mut(idx) {
                                                        if let Ok(at) = event_target_value(&e).parse::<i64>() {
                                                            transport.arrival_time = at;
                                                        }
                                                    }
                                                });
                                            }
                                        />
                                        <input
                                            type="number"
                                            placeholder="Capacity"
                                            prop:value=move || transports.get().get(idx).map(|t| t.capacity.to_string()).unwrap_or_default()
                                            on:input=move |e| {
                                                set_transports.update(|t| {
                                                    if let Some(transport) = t.get_mut(idx) {
                                                        if let Ok(cap) = event_target_value(&e).parse::<u64>() {
                                                            transport.capacity = cap;
                                                        }
                                                    }
                                                });
                                            }
                                        />
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

            // Call backend to solve
            match crate::solver_handler::solve_network_from_input(&input, &network_choice) {
                Ok(response) => {
                    // Include all response fields: stats + solution data
                    let stats_json = json!({
                        "num_variables": response.num_variables,
                        "num_constraints": response.num_constraints,
                        "num_commodities": response.num_commodities,
                        "num_spaces": response.num_spaces,
                        "num_transports": response.num_transports,
                        "objective_value": response.objective_value,
                        "status": response.status,
                        "solution_data": response.solution_data,
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
            <h2>"Network Statistics & Solution"</h2>

            {move || {
                stats.get().map(|s| {
                    // Extract values outside closures to avoid lifetime issues
                    let obj_val = s.get("objective_value").and_then(|v| v.as_f64());
                    let status = s.get("status").and_then(|v| v.as_str()).map(|s| s.to_string());

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

                            {obj_val.map(|ov| {
                                view! {
                                    <div class="stat-card solution">
                                        <div class="stat-value">
                                            {format!("{:.2}", ov)}
                                        </div>
                                        <div class="stat-label">"Objective Value"</div>
                                    </div>
                                }
                            })}

                            {status.map(|st| {
                                view! {
                                    <div class="stat-card solution">
                                        <div class="stat-value">{st}</div>
                                        <div class="stat-label">"Status"</div>
                                    </div>
                                }
                            })}
                        </div>
                    }
                })
            }}

            {move || {
                stats.get().and_then(|s| {
                    s.get("solution_data").map(|sol_data| {
                        view! {
                            <div class="solution-details">
                                <h3>"Commodity Routing"</h3>
                                <div class="commodities-list">
                                    {if let Some(commodities) = sol_data.get("commodity_solutions").and_then(|v| v.as_array()) {
                                        commodities.iter().enumerate().map(|(idx, commodity)| {
                                            let com_id = commodity.get("commodity_id").and_then(|v| v.as_u64()).unwrap_or(idx as u64);
                                            let total_flow = commodity.get("total_flow").and_then(|v| v.as_u64()).unwrap_or(0);
                                            let num_paths = commodity.get("paths").and_then(|v| v.as_array().map(|a| a.len())).unwrap_or(0);

                                            view! {
                                                <div class="commodity-item">
                                                    <span class="commodity-label">{format!("Commodity {}", com_id)}</span>
                                                    <span class="commodity-stat">{format!("Flow: {} | Paths: {}", total_flow, num_paths)}</span>
                                                </div>
                                            }
                                        }).collect_view()
                                    } else {
                                        view! { <p>"No routing data"</p> }.into_view()
                                    }}
                                </div>

                                <h3>"Transport Utilization"</h3>
                                <div class="transports-list">
                                    {if let Some(transports) = sol_data.get("transport_utilizations").and_then(|v| v.as_array()) {
                                        transports.iter().enumerate().map(|(idx, transport)| {
                                            let t_id = transport.get("transport_id").and_then(|v| v.as_u64()).unwrap_or(idx as u64);
                                            let total_load = transport.get("total_load").and_then(|v| v.as_u64()).unwrap_or(0);
                                            let num_com = transport.get("num_commodities").and_then(|v| v.as_u64()).unwrap_or(0);

                                            view! {
                                                <div class="transport-item">
                                                    <span class="transport-label">{format!("Transport {}", t_id)}</span>
                                                    <span class="transport-stat">{format!("Load: {} | Commodities: {}", total_load, num_com)}</span>
                                                </div>
                                            }
                                        }).collect_view()
                                    } else {
                                        view! { <p>"No utilization data"</p> }.into_view()
                                    }}
                                </div>

                                {sol_data.get("total_flow_routed").and_then(|v| v.as_u64()).map(|tf| {
                                    view! {
                                        <div class="flow-summary">
                                            <strong>"Total Flow Routed: " {tf}</strong>
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    })
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
