use crate::components::*;
use crate::storage;
use crate::types::*;
use leptos::prelude::*;

fn parse_list(s: &str) -> Vec<f64> {
    s.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f64>().ok())
        .filter(|n| *n > 0.0)
        .collect()
}

fn generate_rows(speeds_str: &str, accels_str: &str, rows: RwSignal<Vec<RowData>>) {
    let speeds = parse_list(speeds_str);
    let accels = parse_list(accels_str);
    if speeds.is_empty() || accels.is_empty() {
        rows.set(Vec::new());
        return;
    }
    let mut new_rows = Vec::new();
    for &accel in &accels {
        for &speed in &speeds {
            new_rows.push(RowData {
                speed,
                accel,
                flow: RwSignal::new(String::new()),
                pa: RwSignal::new(String::new()),
            });
        }
    }
    rows.set(new_rows);
}

fn fill_from_config(rows: &[RowData], config: &SavedConfig) {
    for row in rows {
        let key = format!("{}-{}", row.speed, row.accel);
        if let Some(cell) = config
            .cells
            .iter()
            .find(|c| format!("{}-{}", c.speed, c.accel) == key)
        {
            row.flow.set(cell.flow.clone());
            row.pa.set(cell.pa.clone());
        }
    }
}

fn collect_cells(rows: &[RowData]) -> Vec<SerializedCell> {
    rows.iter()
        .map(|row| SerializedCell {
            speed: row.speed,
            accel: row.accel,
            flow: row.flow.get(),
            pa: row.pa.get(),
        })
        .collect()
}

#[component]
pub fn App() -> impl IntoView {
    let speeds = RwSignal::new("50, 100, 150, 200".to_string());
    let accels = RwSignal::new("1000, 2000, 4000".to_string());
    let rows = RwSignal::new(Vec::<RowData>::new());
    let profiles = RwSignal::new({
        let mut names: Vec<String> = storage::load_all().into_keys().collect();
        names.sort();
        names
    });
    let selected_profile = RwSignal::new(String::new());
    let name_input = RwSignal::new(String::new());

    let config_text = Memo::new(move |_| {
        rows.with(|r| {
            r.iter()
                .map(|rd| format!("{}, {}, {}", rd.pa.get(), rd.flow.get(), rd.accel))
                .collect::<Vec<_>>()
                .join("\n")
        })
    });

    let all_filled = Memo::new(move |_| {
        let r = rows.get();
        if r.is_empty() {
            return false;
        }
        r.iter().all(|rd| {
            !rd.flow.get().trim().is_empty() && !rd.pa.get().trim().is_empty()
        })
    });

    let on_load = {
        let rows = rows.clone();
        let speeds = speeds.clone();
        let accels = accels.clone();
        move |config: SavedConfig| {
            speeds.set(config.speeds.clone());
            accels.set(config.accelerations.clone());
            generate_rows(&config.speeds, &config.accelerations, rows);
            fill_from_config(&rows.get(), &config);
        }
    };

    let on_save = {
        let rows = rows.clone();
        let name_input = name_input.clone();
        let profiles = profiles.clone();
        let selected_profile = selected_profile.clone();
        let speeds = speeds.clone();
        let accels = accels.clone();
        move || {
            let name = name_input.get();
            let name = name.trim().to_string();
            if name.is_empty() {
                return;
            }
            let all = storage::load_all();
            if all.contains_key(&name) {
                let proceed = web_sys::window()
                    .and_then(|w| {
                        w.confirm_with_message(&format!(
                            "Profile \"{}\" already exists. Overwrite?",
                            name
                        ))
                        .ok()
                    })
                    .unwrap_or(false);
                if !proceed {
                    return;
                }
            }
            drop(all);
            let cells = collect_cells(&rows.get());
            let config = SavedConfig {
                speeds: speeds.get(),
                accelerations: accels.get(),
                cells,
            };
            storage::save_one(&name, &config);
            let mut names: Vec<String> = storage::load_all().into_keys().collect();
            names.sort();
            profiles.set(names);
            selected_profile.set(name.clone());
        }
    };

    let on_delete = {
        let selected_profile = selected_profile.clone();
        let name_input = name_input.clone();
        let profiles = profiles.clone();
        move || {
            let name = selected_profile.get();
            if name.is_empty() {
                return;
            }
            let proceed = web_sys::window()
                .and_then(|w| {
                    w.confirm_with_message(&format!("Delete profile \"{}\"?", name))
                        .ok()
                })
                .unwrap_or(false);
            if !proceed {
                return;
            }
            storage::delete_one(&name);
            selected_profile.set(String::new());
            name_input.set(String::new());
            let mut names: Vec<String> = storage::load_all().into_keys().collect();
            names.sort();
            profiles.set(names);
        }
    };

    view! {
        <h1>"Adaptive Pressure Advance"</h1>
        <SavedConfigsSection
            profiles=profiles
            selected_profile=selected_profile
            name_input=name_input
            on_load=on_load
            on_save=on_save
            on_delete=on_delete
        />
        <ConfigTable
            speeds=speeds
            accels=accels
            rows=rows
        />
        <ConfigOutput
            config_text=config_text
            all_filled=all_filled
        />
    }
}
