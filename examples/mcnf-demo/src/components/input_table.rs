use leptos::*;
use std::rc::Rc;

pub type RemoveItemFn = Rc<dyn Fn(usize)>;
pub type InputValueFn = Rc<dyn Fn() -> String>;
pub type InputHandlerFn = Rc<dyn Fn(ev::Event)>;

#[derive(Clone)]
pub struct InputCell {
    pub input_type: &'static str,
    pub placeholder: &'static str,
    pub value: InputValueFn,
    pub on_input: InputHandlerFn,
    pub step: Option<&'static str>,
}

impl InputCell {
    pub fn new(
        input_type: &'static str,
        placeholder: &'static str,
        value: impl Fn() -> String + 'static,
        on_input: impl Fn(ev::Event) + 'static,
    ) -> Self {
        Self {
            input_type,
            placeholder,
            value: Rc::new(value),
            on_input: Rc::new(on_input),
            step: None,
        }
    }

    pub fn with_step(mut self, step: &'static str) -> Self {
        self.step = Some(step);
        self
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
                            {remove_item.is_some().then(|| view! { <th class="form-table-action-col">""</th> })}
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
                                                    let value = Rc::clone(&cell.value);
                                                    let on_input = Rc::clone(&cell.on_input);
                                                    match cell.step {
                                                        Some(step) => view! {
                                                            <td>
                                                                <input
                                                                    type={cell.input_type}
                                                                    placeholder={cell.placeholder}
                                                                    step={step}
                                                                    prop:value=move || value()
                                                                    on:input=move |e| on_input(e)
                                                                />
                                                            </td>
                                                        }
                                                        .into_view(),
                                                        None => view! {
                                                            <td>
                                                                <input
                                                                    type={cell.input_type}
                                                                    placeholder={cell.placeholder}
                                                                    prop:value=move || value()
                                                                    on:input=move |e| on_input(e)
                                                                />
                                                            </td>
                                                        }
                                                        .into_view(),
                                                    }
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
                                                            "✖"
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
