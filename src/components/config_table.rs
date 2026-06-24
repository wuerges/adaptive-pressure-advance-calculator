use crate::types::RowData;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn parse_list(s: &str) -> Vec<f64> {
    s.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f64>().ok())
        .filter(|n| *n > 0.0)
        .collect()
}

fn query_sel(el: &web_sys::Element, sel: &str) -> Option<HtmlElement> {
    el.query_selector(sel)
        .ok()
        .flatten()
        .and_then(|e| e.dyn_into::<HtmlElement>().ok())
}

#[component]
pub fn ConfigTable(
    speeds: RwSignal<String>,
    accels: RwSignal<String>,
    rows: RwSignal<Vec<RowData>>,
) -> impl IntoView {
    let generate = move |_| {
        let speeds_list = parse_list(&speeds.get());
        let accels_list = parse_list(&accels.get());
        if speeds_list.is_empty() || accels_list.is_empty() {
            rows.set(Vec::new());
            return;
        }
        let mut new_rows = Vec::new();
        for &accel in &accels_list {
            for &speed in &speeds_list {
                new_rows.push(RowData {
                    speed,
                    accel,
                    flow: RwSignal::new(String::new()),
                    pa: RwSignal::new(String::new()),
                });
            }
        }
        rows.set(new_rows);
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() != "Tab" {
            return;
        }

        let target = match ev.target().and_then(|t| t.dyn_ref::<HtmlElement>().cloned()) {
            Some(el) => el,
            None => return,
        };

        let is_flow = target.class_list().contains("flow-input");
        let is_pa = target.class_list().contains("pa-input");
        if !is_flow && !is_pa {
            return;
        }

        ev.prevent_default();

        let row = match target.closest("tr").ok().flatten() {
            Some(r) => r,
            None => return,
        };

        let focused = if is_flow {
            if ev.shift_key() {
                // Shift+Tab on flow: go to pa in previous row
                match row.previous_element_sibling().and_then(|s| {
                    s.dyn_into::<HtmlElement>().ok()
                }) {
                    Some(prev) => query_sel(&prev, ".pa-input"),
                    None => None,
                }
            } else {
                // Tab on flow: go to pa in same row
                query_sel(&row, ".pa-input")
            }
        } else {
            // is_pa
            if ev.shift_key() {
                // Shift+Tab on pa: go to flow in same row
                query_sel(&row, ".flow-input")
            } else {
                // Tab on pa: go to flow in next row
                match row.next_element_sibling().and_then(|s| {
                    s.dyn_into::<HtmlElement>().ok()
                }) {
                    Some(next) => query_sel(&next, ".flow-input"),
                    None => None,
                }
            }
        };

        if let Some(el) = focused {
            let _ = el.focus();
        }
    };

    view! {
        <div class="input-section">
            <div class="input-group">
                <label>"Speeds"</label>
                <input
                    type="text"
                    prop:value=move || speeds.get()
                    on:input=move |ev| speeds.set(event_target_value(&ev))
                />
            </div>
            <div class="input-group">
                <label>"Accelerations"</label>
                <input
                    type="text"
                    prop:value=move || accels.get()
                    on:input=move |ev| accels.set(event_target_value(&ev))
                />
            </div>
            <button on:click=generate>"Generate Table"</button>
        </div>

        <div class="table-wrapper">
            <table>
                <thead>
                    <tr>
                        <th>"Speed"</th>
                        <th>"Flow"</th>
                        <th>"Acceleration"</th>
                        <th>"PA"</th>
                        <th>"Model values"</th>
                    </tr>
                </thead>
                <tbody on:keydown=handle_keydown tabindex="-1">
                    {move || {
                        let rows_val = rows.get();
                        if rows_val.is_empty() {
                            return vec![
                                view! {
                                    <tr>
                                        <td colspan="5" class="empty-state">
                                            "Enter valid speeds and accelerations, then click Generate."
                                        </td>
                                    </tr>
                                }.into_any()
                            ];
                        }
                        rows_val
                            .into_iter()
                            .map(|row| {
                                view! {
                                    <tr>
                                        <td>{row.speed}</td>
                                        <td>
                                            <input
                                                type="text"
                                                class="flow-input"
                                                placeholder="—"
                                                prop:value=move || row.flow.get()
                                                on:input=move |ev| row.flow.set(event_target_value(&ev))
                                            />
                                        </td>
                                        <td>{row.accel}</td>
                                        <td>
                                            <input
                                                type="text"
                                                class="pa-input"
                                                placeholder="—"
                                                prop:value=move || row.pa.get()
                                                on:input=move |ev| row.pa.set(event_target_value(&ev))
                                            />
                                        </td>
                                        <td class="model-values">
                                            {move || format!("{}, {}, {}", row.pa.get(), row.flow.get(), row.accel)}
                                        </td>
                                    </tr>
                                }
                            })
                            .map(|v| v.into_any())
                            .collect()
                    }}
                </tbody>
            </table>
        </div>
    }
}
