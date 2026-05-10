use crate::app::data::runtime::{AppRunningData, TimeData};
use crate::app::systems::time::clock;

pub fn default_runtime_data() -> AppRunningData {
    AppRunningData {
        playing: false,
        points: Vec::new(),
        last_click_time: 0,
        last_subdivision_time: 0,
        last_click_accent: 0,
        last_tap_tempo_click: 0,
        tempo: 120.0,
        time_data: default_time_data(),
        selected_log_index: 0,
        beat_menu_state: 0,
        settings_menu_state: 0,
        pending_delete_log: None,
        pending_log_title: String::new(),
        pending_title_action: None,
    }
}

pub fn default_time_data() -> TimeData {
    let now = clock::current_time();

    TimeData {
        time: now,
        start_time: now,
        time_since_start: 0,
        delta_time: 0,
        paused_time: 0,
        calculated_time_since_start: 0,
    }
}
