use crate::storage;
use crate::types::SavedConfig;
use leptos::prelude::*;

#[component]
pub fn SavedConfigsSection<FLoad, FSave, FDelete>(
    profiles: RwSignal<Vec<String>>,
    selected_profile: RwSignal<String>,
    name_input: RwSignal<String>,
    on_load: FLoad,
    on_save: FSave,
    on_delete: FDelete,
) -> impl IntoView
where
    FLoad: Fn(SavedConfig) + 'static,
    FSave: Fn() + 'static,
    FDelete: Fn() + 'static,
{
    let on_select = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        selected_profile.set(val.clone());
        name_input.set(val.clone());
        if !val.is_empty() {
            let all = storage::load_all();
            if let Some(config) = all.get(&val) {
                on_load(config.clone());
            }
        }
    };

    let on_save_click = move |_| on_save();
    let on_delete_click = move |_| on_delete();

    view! {
        <div class="saved-configs">
            <h2>"Saved Configs"</h2>
            <div class="configs-row">
                <select
                    prop:value=move || selected_profile.get()
                    on:change=on_select
                >
                    <option value="">"-- Save new --"</option>
                    {move || {
                        profiles.get().into_iter().map(|name| {
                            let n = name.clone();
                            view! {
                                <option value=n>{name}</option>
                            }
                        }).collect_view()
                    }}
                </select>
                <button
                    class="btn-danger"
                    disabled=move || selected_profile.get().is_empty()
                    on:click=on_delete_click
                >
                    "Delete"
                </button>
            </div>
            <div class="configs-row">
                <input
                    type="text"
                    placeholder="Profile name..."
                    prop:value=move || name_input.get()
                    on:input=move |ev| name_input.set(event_target_value(&ev))
                />
                <button class="btn-save" on:click=on_save_click>
                    "Save"
                </button>
            </div>
        </div>
    }
}
