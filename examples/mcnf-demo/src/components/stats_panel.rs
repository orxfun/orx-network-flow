use leptos::*;
use serde_json::Value;

#[component]
pub fn StatsPanel(stats: ReadSignal<Option<Value>>) -> impl IntoView {
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
                                        <div class="stat-label">"Lost Revenue"</div>
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
                        let network_dot = esd
                            .get("network_dot")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_default();
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
                                <div class="solution-header">
                                    <h3>"Solution Analysis"</h3>
                                    <div class="solution-meta">
                                        <span class="meta-badge">"Total flow routed: " <strong>{total_flow}</strong></span>
                                    </div>
                                </div>

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

                                {
                                    let selected_dot = network_dot.clone();
                                    move || {
                                        if view_mode.get() != "graph" {
                                            return view! { <div></div> }.into_view();
                                        }
                                        let dot_src = selected_dot.clone();
                                        let container_id = "graph-network";
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

                                {
                                    let commodity_details_c = commodity_details.clone();
                                    let transport_details_c = transport_details.clone();
                                    move || {
                                        if perspective.get() != "commodity" || view_mode.get() != "tabular" {
                                            return view! { <div></div> }.into_view();
                                        }
                                        let items = commodity_details_c.clone();
                                        let t_items = transport_details_c.clone();
                                        view! {
                                            <div class="perspective-view">
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

                                {
                                    let transport_details_t = transport_details.clone();
                                    let commodity_details_t = commodity_details.clone();
                                    move || {
                                        if perspective.get() != "transport" || view_mode.get() != "tabular" {
                                            return view! { <div></div> }.into_view();
                                        }
                                        let t_items = transport_details_t.clone();
                                        let c_items = commodity_details_t.clone();
                                        view! {
                                            <div class="perspective-view">
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
