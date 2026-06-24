use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct RowData {
    pub speed: f64,
    pub accel: f64,
    pub flow: RwSignal<String>,
    pub pa: RwSignal<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedConfig {
    pub speeds: String,
    pub accelerations: String,
    pub cells: Vec<SerializedCell>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SerializedCell {
    pub speed: f64,
    pub accel: f64,
    pub flow: String,
    pub pa: String,
}
