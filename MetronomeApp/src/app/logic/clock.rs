extern crate chrono;
use chrono::prelude::DateTime;
use chrono::{TimeZone, Utc};
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

/// Formats a Unix timestamp in milliseconds into a human-readable date string.
///
/// # Arguments
///
/// * `date` - A `u128` Unix timestamp in **milliseconds** since the Unix epoch.
/// * `format` - An optional format string following `chrono::format` syntax.  
///              If `None` is provided, defaults to `"%d-%m-%Y %H:%M:%S"`.
///
/// # Returns
///
/// A `String` containing the formatted date and time in UTC.
///
/// # Example
///
/// ```
/// let now = std::time::SystemTime::now()
///     .duration_since(std::time::UNIX_EPOCH)
///     .unwrap()
///     .as_millis();
///
/// let default = format_date(now, None); // "08-06-2025 14:45:23"
/// let custom = format_date(now, Some("%Y/%m/%d")); // "2025/06/08"
/// ```
///
/// # Panics
///
/// Panics if the timestamp cannot be converted into a valid UTC time.
pub fn format_date(date: u128, format: Option<&str>) -> String {
    // Convert milliseconds to seconds and nanoseconds
    let seconds = (date / 1000) as i64;
    let nanos = ((date % 1000) * 1_000_000) as u32;

    // Create DateTime<Utc>
    let datetime: DateTime<Utc> = Utc
        .timestamp_opt(seconds, nanos)
        .single()
        .expect("Invalid timestamp");

    // Use provided format or default to "%d-%m-%Y %H:%M:%S"
    let format_str = format.unwrap_or("%d-%m-%Y %H:%M:%S");
    datetime.format(format_str).to_string()
}
