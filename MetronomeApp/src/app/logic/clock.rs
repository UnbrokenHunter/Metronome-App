extern crate chrono;
use chrono::prelude::DateTime;
use chrono::{Datelike, TimeZone, Utc, Weekday};
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

pub fn format_date(date: u128, format: Option<&str>) -> String {
    // Convert milliseconds to seconds and nanoseconds
    let seconds = (date / 1000) as i64;
    let nanos = ((date % 1000) * 1_000_000) as u32;

    // Create DateTime<Utc>
    let datetime: DateTime<Utc> = Utc
        .timestamp_opt(seconds, nanos)
        .single()
        .expect("Invalid timestamp");

    if let Some(fmt) = format {
        // Use provided format if given
        return datetime.format(fmt).to_string();
    }

    // Default format: "June 10th, 2025"
    let day = datetime.day();
    let suffix = match day {
        11 | 12 | 13 => "th", // special case
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    format!(
        "{} {}{}, {}",
        datetime.format("%B"), // Full month name
        day,
        suffix,
        datetime.year()
    )
}

fn full_weekday_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

pub fn weekday_from_unix_ms(unix_ms: u128) -> &'static str {
    let seconds = (unix_ms / 1000) as i64;
    let nanos = ((unix_ms % 1000) * 1_000_000) as u32;

    let datetime = Utc
        .timestamp_opt(seconds, nanos)
        .single()
        .expect("Invalid timestamp");

    full_weekday_name(datetime.weekday())
}
