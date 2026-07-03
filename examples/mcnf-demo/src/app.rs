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
    let (view_mode, set_view_mode) = create_signal("tabular".to_string());
    let (perspective, set_perspective) = create_signal("commodity".to_string());
    let (focused_commodity, set_focused_commodity) = create_signal::<Option<u64>>(None);
    let (focused_transport, set_focused_transport) = create_signal::<Option<u64>>(None);

    view! {
        <div class="stats-panel">
            <h2>"Network Statistics & Solution"</h2>

            {move || {
                stats.get().map(|s| {
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
                    s.get("enhanced_solution_data").map(|esd| {
                        let total_flow = esd.get("total_flow_routed").and_then(|v| v.as_u64()).unwrap_or(0);
                        let commodity_details = esd.get("commodity_details")
                            .and_then(|v| v.as_array())
                            .cloned()
                            .unwrap_or_default();
                        let transport_details = esd.get("transport_details")
                            .and_then(|v| v.as_array())
                            .cloned()
                            .unwrap_or_default();

                        view! {
                            <div class="solution-section">
                                // ── Header bar ──────────────────────────────
                                <div class="solution-header">
                                    <h3>"Solution Analysis"</h3>
                                    <div class="solution-meta">
                                        <span class="meta-badge">"Total flow routed: " <strong>{total_flow}</strong></span>
                                    </div>
                                </div>

                                // ── View mode toggle (Tabular / Graph) ──────
                                <div class="view-toggle">
                                    <button
                                        class=move || if view_mode.get() == "tabular" { "toggle-btn active" } else { "toggle-btn" }
                                        on:click=move |_| set_view_mode.set("tabular".to_string())
                                    >"Tabular"</button>
                                    <button
                                        class=move || if view_mode.get() == "graph" { "toggle-btn active" } else { "toggle-btn" }
                                        on:click=move |_| set_view_mode.set("graph".to_string())
                                    >"Graph"</button>
                                </div>

                                // ── Perspective toggle ───────────────────────
                                <div class="perspective-toggle">
                                    <button
                                        class=move || if perspective.get() == "commodity" { "toggle-btn active" } else { "toggle-btn" }
                                        on:click=move |_| {
                                            set_perspective.set("commodity".to_string());
                                            set_focused_commodity.set(None);
                                        }
                                    >"Commodity Perspective"</button>
                                    <button
                                        class=move || if perspective.get() == "transport" { "toggle-btn active" } else { "toggle-btn" }
                                        on:click=move |_| {
                                            set_perspective.set("transport".to_string());
                                            set_focused_transport.set(None);
                                        }
                                    >"Transport Perspective"</button>
                                </div>

                                // ── Graph view ──────────────────────────────
                                {
                                    let commodity_dot_g = esd.get("commodity_dot").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                    let transport_dot_g = esd.get("transport_dot").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                    move || {
                                        if view_mode.get() != "graph" { return view! { <div></div> }.into_view(); }
                                        let dot_src = if perspective.get() == "transport" {
                                            transport_dot_g.clone()
                                        } else {
                                            commodity_dot_g.clone()
                                        };
                                        let container_id = if perspective.get() == "transport" {
                                            "graph-transport"
                                        } else {
                                            "graph-commodity"
                                        };
                                        let dot_for_effect = dot_src.clone();
                                        create_effect(move |_| {
                                            let dot = dot_for_effect.clone();
                                            let cid = container_id;
                                            leptos::request_animation_frame(move || {
                                                let _ = js_sys::eval(&format!(
                                                    "window.renderDot('{}', {})",
                                                    cid,
                                                    serde_json::to_string(&dot).unwrap_or_default()
                                                ));
                                            });
                                        });
                                        view! {
                                            <div class="graph-view">
                                                <div id={container_id} class="graph-container">
                                                    <p class="graph-loading">"Rendering graph..."</p>
                                                </div>
                                            </div>
                                        }.into_view()
                                    }
                                }

                                // ── Commodity perspective ────────────────────
                                {
                                    let commodity_details_c = commodity_details.clone();
                                    let transport_details_c = transport_details.clone();
                                    move || {
                                        if perspective.get() != "commodity" || view_mode.get() != "tabular" { return view! { <div></div> }.into_view(); }
                                        let items = commodity_details_c.clone();
                                        let t_items = transport_details_c.clone();
                                        view! {
                                            <div class="perspective-view">
                                                // Focus selector
                                                <div class="focus-bar">
                                                    <label>"Focus on commodity: "</label>
                                                    <select on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_focused_commodity.set(val.parse::<u64>().ok());
                                                    }>
                                                        <option value="">"— All commodities —"</option>
                                                        {items.iter().map(|c| {
                                                            let id = c.get("commodity_id").and_then(|v| v.as_u64()).unwrap_or(0);
                                                            let ori = c.get("origin_space").and_then(|v| v.as_str()).unwrap_or("?");
                                                            let des = c.get("destination_space").and_then(|v| v.as_str()).unwrap_or("?");
                                                            view! { <option value={id.to_string()}>{format!("C{}: {}→{}", id, ori, des)}</option> }
                                                        }).collect_view()}
                                                    </select>
                                                    {move || focused_commodity.get().map(|_| view! {
                                                        <button class="clear-focus" on:click=move |_| set_focused_commodity.set(None)>"✕ Clear"</button>
                                                    })}
                                                </div>

                                                // Commodity table
                                                <table class="solution-table">
                                                    <thead>
                                                        <tr>
                                                            <th>"ID"</th>
                                                            <th>"Origin"</th>
                                                            <th>"Destination"</th>
                                                            <th>"Total Flow"</th>
                                                            <th>"Transports Used"</th>
                                                            <th>"Paths"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {items.iter().filter_map(|c| {
                                                            let c_id = c.get("commodity_id").and_then(|v| v.as_u64()).unwrap_or(0);
                                                            if let Some(fc) = focused_commodity.get() {
                                                                if c_id != fc { return None; }
                                                            }
                                                            let ori = c.get("origin_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                            let des = c.get("destination_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                            let total_flow = c.get("total_flow").and_then(|v| v.as_u64()).unwrap_or(0);
                                                            let t_ids = c.get("transport_ids").and_then(|v| v.as_array())
                                                                .map(|arr| arr.iter().filter_map(|v| v.as_u64()).map(|n| n.to_string()).collect::<Vec<_>>().join(", "))
                                                                .unwrap_or_default();
                                                            let paths = c.get("paths").and_then(|v| v.as_array()).cloned().unwrap_or_default();
                                                            let num_paths = paths.len();
                                                            let is_focused = focused_commodity.get().map(|fc| fc == c_id).unwrap_or(false);

                                                            Some(view! {
                                                                <tr class=if is_focused { "focused-row" } else { "" }
                                                                    on:click=move |_| set_focused_commodity.set(if is_focused { None } else { Some(c_id) })>
                                                                    <td><strong>{format!("C{}", c_id)}</strong></td>
                                                                    <td>{ori}</td>
                                                                    <td>{des}</td>
                                                                    <td class="num-cell">{total_flow}</td>
                                                                    <td class="mono-cell">{t_ids}</td>
                                                                    <td class="num-cell">{num_paths}</td>
                                                                </tr>
                                                                // Expanded path rows when focused
                                                                {if is_focused {
                                                                    paths.iter().enumerate().map(|(pi, path)| {
                                                                        let flow = path.get("flow").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let tp = path.get("transport_path").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                        let sp = path.get("space_path").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                        let vp = path.get("vertex_path").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                        view! {
                                                                            <tr class="path-row">
                                                                                <td colspan="2" class="path-idx">{format!("  Path {}", pi + 1)}</td>
                                                                                <td>"Flow: " <strong>{flow}</strong></td>
                                                                                <td colspan="3">
                                                                                    <span class="path-tag transport-tag">{tp}</span>
                                                                                    " · "
                                                                                    <span class="path-tag space-tag">{sp}</span>
                                                                                    " · "
                                                                                    <span class="path-tag vertex-tag">{vp}</span>
                                                                                </td>
                                                                            </tr>
                                                                        }
                                                                    }).collect_view().into_view()
                                                                } else {
                                                                    view! {}.into_view()
                                                                }}
                                                            })
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>

                                                // Transport cross-reference when a commodity is focused
                                                {move || focused_commodity.get().map(|fc_id| {
                                                    let relevant_transports: Vec<_> = t_items.iter().filter(|t| {
                                                        t.get("assigned_commodities").and_then(|v| v.as_array())
                                                            .map(|arr| arr.iter().any(|ca| ca.get("commodity_id").and_then(|v| v.as_u64()) == Some(fc_id)))
                                                            .unwrap_or(false)
                                                    }).cloned().collect();

                                                    if relevant_transports.is_empty() { return view! {}.into_view(); }

                                                    view! {
                                                        <div class="cross-ref">
                                                            <h4>{format!("Transports used by Commodity {}", fc_id)}</h4>
                                                            <table class="solution-table cross-ref-table">
                                                                <thead>
                                                                    <tr>
                                                                        <th>"Transport"</th>
                                                                        <th>"Route"</th>
                                                                        <th>"Dep/Arr"</th>
                                                                        <th>"Flow on this transport"</th>
                                                                        <th>"Capacity"</th>
                                                                        <th>"Utilization"</th>
                                                                    </tr>
                                                                </thead>
                                                                <tbody>
                                                                    {relevant_transports.iter().map(|t| {
                                                                        let t_id = t.get("transport_id").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let ori = t.get("origin_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                        let des = t.get("destination_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                        let dep = t.get("departure_time").and_then(|v| v.as_i64()).unwrap_or(0);
                                                                        let arr = t.get("arrival_time").and_then(|v| v.as_i64()).unwrap_or(0);
                                                                        let cap = t.get("capacity").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let util = t.get("utilization_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                                                        let flow_on = t.get("assigned_commodities").and_then(|v| v.as_array())
                                                                            .and_then(|arr| arr.iter().find(|ca| ca.get("commodity_id").and_then(|v| v.as_u64()) == Some(fc_id)))
                                                                            .and_then(|ca| ca.get("assigned_flow").and_then(|v| v.as_u64()))
                                                                            .unwrap_or(0);
                                                                        let util_pct = (util * 100.0).round() as u64;
                                                                        let util_class = if util >= 0.8 { "util-high" } else if util >= 0.4 { "util-mid" } else { "util-low" };
                                                                        view! {
                                                                            <tr>
                                                                                <td><strong>{format!("T{}", t_id)}</strong></td>
                                                                                <td>{format!("{}→{}", ori, des)}</td>
                                                                                <td class="mono-cell">{format!("{}→{}", dep, arr)}</td>
                                                                                <td class="num-cell">{flow_on}</td>
                                                                                <td class="num-cell">{cap}</td>
                                                                                <td><span class=util_class>{format!("{}%", util_pct)}</span></td>
                                                                            </tr>
                                                                        }
                                                                    }).collect_view()}
                                                                </tbody>
                                                            </table>
                                                        </div>
                                                    }.into_view()
                                                })}
                                            </div>
                                        }.into_view()
                                    }
                                }

                                // ── Transport perspective ────────────────────
                                {
                                    let transport_details_t = transport_details.clone();
                                    let commodity_details_t = commodity_details.clone();
                                    move || {
                                        if perspective.get() != "transport" || view_mode.get() != "tabular" { return view! { <div></div> }.into_view(); }
                                        let t_items = transport_details_t.clone();
                                        let c_items = commodity_details_t.clone();
                                        view! {
                                            <div class="perspective-view">
                                                // Focus selector
                                                <div class="focus-bar">
                                                    <label>"Focus on transport: "</label>
                                                    <select on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_focused_transport.set(val.parse::<u64>().ok());
                                                    }>
                                                        <option value="">"— All transports —"</option>
                                                        {t_items.iter().map(|t| {
                                                            let id = t.get("transport_id").and_then(|v| v.as_u64()).unwrap_or(0);
                                                            let ori = t.get("origin_space").and_then(|v| v.as_str()).unwrap_or("?");
                                                            let des = t.get("destination_space").and_then(|v| v.as_str()).unwrap_or("?");
                                                            let dep = t.get("departure_time").and_then(|v| v.as_i64()).unwrap_or(0);
                                                            view! { <option value={id.to_string()}>{format!("T{}: {}→{} @t{}", id, ori, des, dep)}</option> }
                                                        }).collect_view()}
                                                    </select>
                                                    {move || focused_transport.get().map(|_| view! {
                                                        <button class="clear-focus" on:click=move |_| set_focused_transport.set(None)>"✕ Clear"</button>
                                                    })}
                                                </div>

                                                // Transport table
                                                <table class="solution-table">
                                                    <thead>
                                                        <tr>
                                                            <th>"ID"</th>
                                                            <th>"Route"</th>
                                                            <th>"Dep / Arr"</th>
                                                            <th>"Utilized"</th>
                                                            <th>"Capacity"</th>
                                                            <th>"Utilization"</th>
                                                            <th>"Commodities"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {t_items.iter().filter_map(|t| {
                                                            let t_id = t.get("transport_id").and_then(|v| v.as_u64()).unwrap_or(0);
                                                            if let Some(ft) = focused_transport.get() {
                                                                if t_id != ft { return None; }
                                                            }
                                                            let ori = t.get("origin_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                            let des = t.get("destination_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                            let dep = t.get("departure_time").and_then(|v| v.as_i64()).unwrap_or(0);
                                                            let arr = t.get("arrival_time").and_then(|v| v.as_i64()).unwrap_or(0);
                                                            let cap = t.get("capacity").and_then(|v| v.as_u64()).unwrap_or(0);
                                                            let utilized = t.get("utilized_capacity").and_then(|v| v.as_u64()).unwrap_or(0);
                                                            let util = t.get("utilization_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                                            let util_pct = (util * 100.0).round() as u64;
                                                            let util_class = if util >= 0.8 { "util-high" } else if util >= 0.4 { "util-mid" } else { "util-low" };
                                                            let assigned_commodities = t.get("assigned_commodities").and_then(|v| v.as_array()).cloned().unwrap_or_default();
                                                            let c_ids = assigned_commodities.iter()
                                                                .filter_map(|ca| ca.get("commodity_id").and_then(|v| v.as_u64()))
                                                                .map(|id| format!("C{}", id))
                                                                .collect::<Vec<_>>()
                                                                .join(", ");
                                                            let is_focused = focused_transport.get().map(|ft| ft == t_id).unwrap_or(false);

                                                            Some(view! {
                                                                <tr class=if is_focused { "focused-row" } else { "" }
                                                                    on:click=move |_| set_focused_transport.set(if is_focused { None } else { Some(t_id) })>
                                                                    <td><strong>{format!("T{}", t_id)}</strong></td>
                                                                    <td>{format!("{}→{}", ori, des)}</td>
                                                                    <td class="mono-cell">{format!("{}→{}", dep, arr)}</td>
                                                                    <td class="num-cell">{utilized}</td>
                                                                    <td class="num-cell">{cap}</td>
                                                                    <td>
                                                                        <div class="util-bar-wrap">
                                                                            <div class="util-bar" style={format!("width:{}%", util_pct.min(100))}></div>
                                                                            <span class=util_class>{format!("{}%", util_pct)}</span>
                                                                        </div>
                                                                    </td>
                                                                    <td class="mono-cell">{c_ids}</td>
                                                                </tr>
                                                                // Expanded commodity rows when focused
                                                                {if is_focused {
                                                                    assigned_commodities.iter().map(|ca| {
                                                                        let ca_id = ca.get("commodity_id").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let ca_flow = ca.get("assigned_flow").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let ca_paths = ca.get("num_paths").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let ca_ori = c_items.iter()
                                                                            .find(|c| c.get("commodity_id").and_then(|v| v.as_u64()) == Some(ca_id))
                                                                            .and_then(|c| {
                                                                                let o = c.get("origin_space").and_then(|v| v.as_str()).unwrap_or("?");
                                                                                let d = c.get("destination_space").and_then(|v| v.as_str()).unwrap_or("?");
                                                                                Some(format!("{}→{}", o, d))
                                                                            })
                                                                            .unwrap_or_default();
                                                                        view! {
                                                                            <tr class="path-row">
                                                                                <td colspan="2" class="path-idx">{format!("  C{}", ca_id)}</td>
                                                                                <td>{ca_ori}</td>
                                                                                <td class="num-cell">{ca_flow}</td>
                                                                                <td colspan="3">{format!("{} path(s) through this transport", ca_paths)}</td>
                                                                            </tr>
                                                                        }
                                                                    }).collect_view().into_view()
                                                                } else {
                                                                    view! {}.into_view()
                                                                }}
                                                            })
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>

                                                // Commodity cross-reference when a transport is focused
                                                {move || focused_transport.get().map(|ft_id| {
                                                    let relevant_commodities: Vec<_> = c_items.iter().filter(|c| {
                                                        c.get("transport_ids").and_then(|v| v.as_array())
                                                            .map(|arr| arr.iter().any(|v| v.as_u64() == Some(ft_id)))
                                                            .unwrap_or(false)
                                                    }).cloned().collect();

                                                    if relevant_commodities.is_empty() { return view! {}.into_view(); }

                                                    view! {
                                                        <div class="cross-ref">
                                                            <h4>{format!("Commodities routed on Transport {}", ft_id)}</h4>
                                                            <table class="solution-table cross-ref-table">
                                                                <thead>
                                                                    <tr>
                                                                        <th>"Commodity"</th>
                                                                        <th>"Route"</th>
                                                                        <th>"Total Flow"</th>
                                                                        <th>"Paths"</th>
                                                                    </tr>
                                                                </thead>
                                                                <tbody>
                                                                    {relevant_commodities.iter().map(|c| {
                                                                        let c_id = c.get("commodity_id").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let ori = c.get("origin_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                        let des = c.get("destination_space").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                        let total_flow = c.get("total_flow").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                        let paths = c.get("paths").and_then(|v| v.as_array()).cloned().unwrap_or_default();
                                                                        let relevant_paths: Vec<_> = paths.iter().filter(|p| {
                                                                            p.get("transport_path").and_then(|v| v.as_str())
                                                                                .map(|tp| tp.split('-').any(|seg| seg.parse::<u64>().ok() == Some(ft_id)))
                                                                                .unwrap_or(false)
                                                                        }).cloned().collect();
                                                                        view! {
                                                                            <tr>
                                                                                <td><strong>{format!("C{}", c_id)}</strong></td>
                                                                                <td>{format!("{}→{}", ori, des)}</td>
                                                                                <td class="num-cell">{total_flow}</td>
                                                                                <td>
                                                                                    {relevant_paths.iter().map(|p| {
                                                                                        let sp = p.get("space_path").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default();
                                                                                        let flow = p.get("flow").and_then(|v| v.as_u64()).unwrap_or(0);
                                                                                        view! { <div><span class="path-tag space-tag">{sp}</span>" (" {flow} ")"</div> }
                                                                                    }).collect_view()}
                                                                                </td>
                                                                            </tr>
                                                                        }
                                                                    }).collect_view()}
                                                                </tbody>
                                                            </table>
                                                        </div>
                                                    }.into_view()
                                                })}
                                            </div>
                                        }.into_view()
                                    }
                                }
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
