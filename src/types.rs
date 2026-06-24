use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub fn parse_list(s: &str) -> Vec<f64> {
    s.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f64>().ok())
        .filter(|n| *n > 0.0 && n.is_finite())
        .collect()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_valid() {
        assert_eq!(parse_list("50, 100, 150"), vec![50.0, 100.0, 150.0]);
    }

    #[test]
    fn parse_list_extra_spaces() {
        assert_eq!(parse_list("  50 , 100 , 150  "), vec![50.0, 100.0, 150.0]);
    }

    #[test]
    fn parse_list_empty() {
        assert_eq!(parse_list(""), Vec::<f64>::new());
    }

    #[test]
    fn parse_list_whitespace_only() {
        assert_eq!(parse_list("   ,  ,  "), Vec::<f64>::new());
    }

    #[test]
    fn parse_list_invalid_values() {
        assert_eq!(parse_list("abc, 50, def"), vec![50.0]);
    }

    #[test]
    fn parse_list_negative_values() {
        assert_eq!(parse_list("-10, 50, 0"), vec![50.0]);
    }

    #[test]
    fn parse_list_single_value() {
        assert_eq!(parse_list("42"), vec![42.0]);
    }

    #[test]
    fn parse_list_rejects_infinity() {
        assert_eq!(parse_list("inf, 50, infinity"), vec![50.0]);
    }
}
