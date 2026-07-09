use leptos::*;

#[component]
pub fn ProblemForm(
    on_built: impl Fn(crate::serialization::ProblemInput) + 'static,
    on_error: impl Fn(String) + 'static,
) -> impl IntoView {
    let (spaces, set_spaces) = create_signal(vec![SpaceInput::default()]);
    let (commodities, set_commodities) = create_signal(vec![]);
    let (transports, set_transports) = create_signal(vec![]);
    let (lost_revenue_items, set_lost_revenue_items) = create_signal::<Vec<(usize, i64)>>(vec![]);

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
            lost_revenue_costs: lost_revenue_items
                .get()
                .into_iter()
                .map(
                    |(commodity_id, cost_per_unit)| crate::serialization::FormLostRevenueItem {
                        commodity_id,
                        cost_per_unit,
                    },
                )
                .collect(),
        };

        on_built(problem_input);
    };

    let load_demo_input = move |_| {
        let (demo_spaces, demo_commodities, demo_transports) = demo_data();
        set_spaces.set(demo_spaces);
        set_commodities.set(demo_commodities);
        set_transports.set(demo_transports);
        set_lost_revenue_items.set(vec![(0, 1), (1, 3), (2, 10), (3, 2), (4, 8)]);
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

            <fieldset>
                <legend>"Lost Revenue Costs (per unrouted unit)"</legend>
                <div class="form-group">
                    {move || {
                        lost_revenue_items.get().into_iter().enumerate().map(|(idx, _)| {
                            view! {
                                <div class="form-row">
                                    <label>"Commodity ID:"</label>
                                    <input
                                        type="number"
                                        placeholder="Commodity ID"
                                        prop:value=move || lost_revenue_items.get().get(idx).map(|(id, _)| id.to_string()).unwrap_or_default()
                                        on:input=move |e| {
                                            set_lost_revenue_items.update(|items| {
                                                if let Some(item) = items.get_mut(idx) {
                                                    if let Ok(id) = event_target_value(&e).parse::<usize>() {
                                                        item.0 = id;
                                                    }
                                                }
                                            });
                                        }
                                    />
                                    <label>"Cost per unit:"</label>
                                    <input
                                        type="number"
                                        placeholder="Revenue per unit"
                                        prop:value=move || lost_revenue_items.get().get(idx).map(|(_, cost)| cost.to_string()).unwrap_or_default()
                                        on:input=move |e| {
                                            set_lost_revenue_items.update(|items| {
                                                if let Some(item) = items.get_mut(idx) {
                                                    if let Ok(cost) = event_target_value(&e).parse::<i64>() {
                                                        item.1 = cost;
                                                    }
                                                }
                                            });
                                        }
                                    />
                                    <button
                                        class="btn-remove"
                                        on:click=move |_| set_lost_revenue_items.update(|items| { items.remove(idx); })
                                    >"Remove"</button>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                    <button class="btn-add" on:click=move |_| set_lost_revenue_items.update(|items| items.push((0, 1)))>
                        "+ Add Lost Revenue Cost"
                    </button>
                </div>
            </fieldset>

            <button class="btn-submit" on:click=on_submit>
                "Build Problem"
            </button>
        </div>
    }
}

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
