use crate::app_data::AppData;
use crate::space_kind::SpaceKind;
use leptos::*;

#[component]
pub fn ViewSpaces() -> impl IntoView {
    let app = expect_context::<AppData>();

    view! {
        <section class="spaces-view">
            <div class="spaces-kind-tabs" role="tablist" aria-label="Space kind">
                <button
                    class="spaces-kind-tabs__button"
                    class=("spaces-kind-tabs__button--active", move || matches!(app.active_space_kind.get(), SpaceKind::Basic))
                    on:click=move |_| app.active_space_kind.set(SpaceKind::Basic)
                    type="button"
                >
                    "Basic"
                </button>
                <button
                    class="spaces-kind-tabs__button"
                    class=("spaces-kind-tabs__button--active", move || matches!(app.active_space_kind.get(), SpaceKind::Euclidean))
                    on:click=move |_| app.active_space_kind.set(SpaceKind::Euclidean)
                    type="button"
                >
                    "Euclidean"
                </button>
                <button
                    class="spaces-kind-tabs__button"
                    class=("spaces-kind-tabs__button--active", move || matches!(app.active_space_kind.get(), SpaceKind::Geographic))
                    on:click=move |_| app.active_space_kind.set(SpaceKind::Geographic)
                    type="button"
                >
                    "Geographic"
                </button>
            </div>

            {move || match app.active_space_kind.get() {
                SpaceKind::Basic => {
                    view! {
                        <div class="spaces-table-wrap">
                            <table class="spaces-table">
                                <thead>
                                    <tr>
                                        <th>"Key"</th>
                                        <th class="spaces-table__actions-col">"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For
                                        each=move || {
                                            app.spaces_basic
                                                .get()
                                                .into_iter()
                                                .enumerate()
                                                .collect::<Vec<_>>()
                                        }
                                        key=|(idx, _)| *idx
                                        children=move |(idx, key)| {
                                            view! {
                                                <tr>
                                                    <td>
                                                        <input
                                                            class="spaces-table__input"
                                                            prop:value=key
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                app.spaces_basic.update(|rows| {
                                                                    if let Some(cell) = rows.get_mut(idx) {
                                                                        *cell = value;
                                                                    }
                                                                });
                                                            }
                                                        />
                                                    </td>
                                                    <td class="spaces-table__actions-cell">
                                                        <button
                                                            class="spaces-table__remove"
                                                            on:click=move |_| {
                                                                app.spaces_basic.update(|rows| {
                                                                    if idx < rows.len() {
                                                                        rows.remove(idx);
                                                                    }
                                                                });
                                                            }
                                                            type="button"
                                                        >
                                                            "Remove"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>

                            <button
                                class="spaces-table__add"
                                on:click=move |_| {
                                    app.spaces_basic.update(|rows| {
                                        rows.push(String::new());
                                    });
                                }
                                type="button"
                            >
                                "Add Row"
                            </button>
                        </div>
                    }
                        .into_view()
                }
                SpaceKind::Euclidean => {
                    view! {
                        <div class="spaces-table-wrap">
                            <table class="spaces-table">
                                <thead>
                                    <tr>
                                        <th>"Key"</th>
                                        <th>"X"</th>
                                        <th>"Y"</th>
                                        <th class="spaces-table__actions-col">"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For
                                        each=move || {
                                            app.spaces_euclidean
                                                .get()
                                                .into_iter()
                                                .enumerate()
                                                .collect::<Vec<_>>()
                                        }
                                        key=|(idx, _)| *idx
                                        children=move |(idx, (key, x, y))| {
                                            view! {
                                                <tr>
                                                    <td>
                                                        <input
                                                            class="spaces-table__input"
                                                            prop:value=key
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                app.spaces_euclidean.update(|rows| {
                                                                    if let Some(cell) = rows.get_mut(idx) {
                                                                        cell.0 = value;
                                                                    }
                                                                });
                                                            }
                                                        />
                                                    </td>
                                                    <td>
                                                        <input
                                                            class="spaces-table__input"
                                                            prop:value=x
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                if let Ok(parsed) = value.parse::<f64>() {
                                                                    app.spaces_euclidean.update(|rows| {
                                                                        if let Some(cell) = rows.get_mut(idx) {
                                                                            cell.1 = parsed;
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                        />
                                                    </td>
                                                    <td>
                                                        <input
                                                            class="spaces-table__input"
                                                            prop:value=y
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                if let Ok(parsed) = value.parse::<f64>() {
                                                                    app.spaces_euclidean.update(|rows| {
                                                                        if let Some(cell) = rows.get_mut(idx) {
                                                                            cell.2 = parsed;
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                        />
                                                    </td>
                                                    <td class="spaces-table__actions-cell">
                                                        <button
                                                            class="spaces-table__remove"
                                                            on:click=move |_| {
                                                                app.spaces_euclidean.update(|rows| {
                                                                    if idx < rows.len() {
                                                                        rows.remove(idx);
                                                                    }
                                                                });
                                                            }
                                                            type="button"
                                                        >
                                                            "Remove"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>

                            <button
                                class="spaces-table__add"
                                on:click=move |_| {
                                    app.spaces_euclidean.update(|rows| {
                                        rows.push((String::new(), 0.0, 0.0));
                                    });
                                }
                                type="button"
                            >
                                "Add Row"
                            </button>
                        </div>
                    }
                        .into_view()
                }
                SpaceKind::Geographic => {
                    view! {
                        <div class="spaces-table-wrap">
                            <table class="spaces-table">
                                <thead>
                                    <tr>
                                        <th>"Key"</th>
                                        <th>"Lat"</th>
                                        <th>"Lon"</th>
                                        <th class="spaces-table__actions-col">"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For
                                        each=move || {
                                            app.spaces_geographic
                                                .get()
                                                .into_iter()
                                                .enumerate()
                                                .collect::<Vec<_>>()
                                        }
                                        key=|(idx, _)| *idx
                                        children=move |(idx, (key, lat, lon))| {
                                            view! {
                                                <tr>
                                                    <td>
                                                        <input
                                                            class="spaces-table__input"
                                                            prop:value=key
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                app.spaces_geographic.update(|rows| {
                                                                    if let Some(cell) = rows.get_mut(idx) {
                                                                        cell.0 = value;
                                                                    }
                                                                });
                                                            }
                                                        />
                                                    </td>
                                                    <td>
                                                        <input
                                                            class="spaces-table__input"
                                                            prop:value=lat
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                if let Ok(parsed) = value.parse::<f64>() {
                                                                    app.spaces_geographic.update(|rows| {
                                                                        if let Some(cell) = rows.get_mut(idx) {
                                                                            cell.1 = parsed;
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                        />
                                                    </td>
                                                    <td>
                                                        <input
                                                            class="spaces-table__input"
                                                            prop:value=lon
                                                            on:input=move |ev| {
                                                                let value = event_target_value(&ev);
                                                                if let Ok(parsed) = value.parse::<f64>() {
                                                                    app.spaces_geographic.update(|rows| {
                                                                        if let Some(cell) = rows.get_mut(idx) {
                                                                            cell.2 = parsed;
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                        />
                                                    </td>
                                                    <td class="spaces-table__actions-cell">
                                                        <button
                                                            class="spaces-table__remove"
                                                            on:click=move |_| {
                                                                app.spaces_geographic.update(|rows| {
                                                                    if idx < rows.len() {
                                                                        rows.remove(idx);
                                                                    }
                                                                });
                                                            }
                                                            type="button"
                                                        >
                                                            "Remove"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>

                            <button
                                class="spaces-table__add"
                                on:click=move |_| {
                                    app.spaces_geographic.update(|rows| {
                                        rows.push((String::new(), 0.0, 0.0));
                                    });
                                }
                                type="button"
                            >
                                "Add Row"
                            </button>
                        </div>
                    }
                        .into_view()
                }
            }}
        </section>
    }
}
