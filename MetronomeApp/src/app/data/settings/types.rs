use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsData {
    pub save_logs: bool,
    pub save_plots: bool,
    pub do_title_popup: bool,
    pub plot_granularity: u8,
    pub min_practice_length: u64,
    pub selected_theme_index: usize,
}