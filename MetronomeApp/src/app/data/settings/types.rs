use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsData {
    pub save_logs: bool,
    pub save_plots: bool,
    pub plot_granularity: u8,
    pub min_practice_length: u64,
    pub color_scheme: ColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub name: String,
    pub override_color: String,
    pub downbeat_color: String,
    pub strong_color: String,
    pub weak_color: String,
    pub off_color: String,
}
