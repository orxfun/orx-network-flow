use leptos::*;
use std::rc::Rc;

pub type RemoveItemFn = Rc<dyn Fn(usize)>;

#[derive(Clone)]
pub struct InputCell {
    pub content: View,
    pub class: &'static str,
}

impl InputCell {
    pub fn new(content: View) -> Self {
        Self { content, class: "" }
    }
}

#[derive(Clone)]
pub struct InputRow {
    pub cells: Vec<InputCell>,
}

impl InputRow {
    pub fn new(cells: Vec<InputCell>) -> Self {
        Self { cells }
    }
}

#[component]
pub fn InputTable<RowFn>(
    headers: Vec<&'static str>,
    add_label: &'static str,
    on_add: impl Fn(ev::MouseEvent) + 'static,
    rows: RowFn,
    #[prop(optional)] remove_item: Option<RemoveItemFn>,
) -> impl IntoView
where
    RowFn: Fn() -> Vec<InputRow> + 'static,
{
    view! {
        <div class="form-group">
            <div class="form-table-wrap">
                <table class="form-table">
                    <thead>
                        <tr>
                            {headers
                                .into_iter()
                                .map(|header| view! { <th>{header}</th> })
                                .collect_view()}
                            {remove_item.is_some().then(|| view! { <th class="form-table-action-col">"Action"</th> })}
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            let remove_item = remove_item.clone();
                            rows()
                                .into_iter()
                                .enumerate()
                                .map(|(idx, row)| {
                                    view! {
                                        <tr>
                                            {row
                                                .cells
                                                .into_iter()
                                                .map(|cell| {
                                                    view! { <td class={cell.class}>{cell.content}</td> }
                                                })
                                                .collect_view()}
                                            {remove_item.as_ref().map(|remove| {
                                                let remove = Rc::clone(remove);
                                                view! {
                                                    <td class="form-table-action-cell">
                                                        <button
                                                            class="btn-remove"
                                                            on:click=move |_| remove(idx)
                                                        >
                                                            "Remove"
                                                        </button>
                                                    </td>
                                                }
                                            })}
                                        </tr>
                                    }
                                })
                                .collect_view()
                        }}
                    </tbody>
                </table>
            </div>
            <button class="btn-add" on:click=on_add>
                {add_label}
            </button>
        </div>
    }
}
