use std::collections::HashMap;
use crate::types::SavedConfig;
use web_sys::window;

const KEY: &str = "apa-configs";

pub fn load_all() -> HashMap<String, SavedConfig> {
    let storage = storage();
    let val = storage
        .get_item(KEY)
        .ok()
        .flatten()
        .unwrap_or_default();
    serde_json::from_str(&val).unwrap_or_default()
}

pub fn save_one(name: &str, config: &SavedConfig) {
    let mut all = load_all();
    all.insert(name.to_string(), config.clone());
    if let Ok(json) = serde_json::to_string(&all) {
        let _ = storage().set_item(KEY, &json);
    }
}

pub fn delete_one(name: &str) {
    let mut all = load_all();
    all.remove(name);
    if let Ok(json) = serde_json::to_string(&all) {
        let _ = storage().set_item(KEY, &json);
    }
}

fn storage() -> web_sys::Storage {
    window()
        .expect("window")
        .local_storage()
        .expect("localStorage")
        .expect("localStorage available")
}
