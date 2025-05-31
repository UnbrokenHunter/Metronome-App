use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::types::TimeData;

pub fn update_time(time_data: &mut TimeData, playing: bool) {
    let unix: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    time_data.delta_time = unix - time_data.time; // Time since last update
    time_data.time = unix; // Set current time
    time_data.time_since_start = time_data.time - time_data.start_time; // Set Time since start

    if !playing {
        time_data.paused_time += time_data.delta_time;
    }
    time_data.calculated_time_since_start = time_data.time_since_start - time_data.paused_time;
}

pub fn format_time(time: u128) -> String {
    let total_seconds = time / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = (time as f64 / 1000.0) % 60.0;
    format!("{:02}:{:02}:{:05.2}", hours, minutes, seconds)
}
