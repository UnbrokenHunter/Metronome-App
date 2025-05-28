use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::types::TimeData;

pub fn update_time(time_data: &mut TimeData) {
    let unix: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    time_data.delta_time = unix - time_data.time; // Time since last update
    time_data.time = unix; // Set current time
    time_data.time_since_start = time_data.time - time_data.start_time; // Set Time since start
}
