extern crate chrono;
use chrono::prelude::DateTime;
use chrono::{Datelike, Local, TimeZone};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::TimeData;

pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

pub fn update_time(time_data: &mut TimeData, playing: bool) {
    let unix: u128 = current_time();

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

/// Formats a Unix timestamp in milliseconds into a readable date string.
///
/// Pass `None` to use the default format.
///
/// Pass `Some("...")` to use a custom chrono date/time format.
///
/// # Custom format examples
///
/// ```text
/// format_date(date, None)                    -> June 10th, 2025 at 3:42 PM
///
/// format_date(date, Some("%B %d, %Y"))        -> June 10, 2025
/// format_date(date, Some("%A, %B %d, %Y"))    -> Tuesday, June 10, 2025
/// format_date(date, Some("%a, %b %d"))        -> Tue, Jun 10
///
/// format_date(date, Some("%Y-%m-%d"))         -> 2025-06-10
/// format_date(date, Some("%m/%d/%Y"))         -> 06/10/2025
/// format_date(date, Some("%Y-%m-%d %H:%M"))   -> 2025-06-10 15:42
///
/// format_date(date, Some("%H:%M"))            -> 15:42
/// format_date(date, Some("%I:%M %p"))         -> 03:42 PM
/// ```
///
/// # Common format codes
///
/// ```text
/// %Y = 4-digit year      2025
/// %y = 2-digit year      25
/// %B = full month        June
/// %b = short month       Jun
/// %m = month number      06
/// %A = full weekday      Tuesday
/// %a = short weekday     Tue
/// %d = day of month      10
/// %H = 24-hour hour      15
/// %I = 12-hour hour      03
/// %M = minute            42
/// %S = second            09
/// %p = AM/PM             PM
/// ```
pub fn format_date(date: u128, format: Option<&str>) -> String {
    let seconds = (date / 1000) as i64;
    let nanos = ((date % 1000) * 1_000_000) as u32;

    let datetime: DateTime<Local> = Local
        .timestamp_opt(seconds, nanos)
        .single()
        .expect("Invalid timestamp");

    let day = datetime.day();
    let day_ordinal = format!("{}{}", day, ordinal_suffix(day));

    if let Some(fmt) = format {
        let placeholder = "__DAY_ORDINAL__";
        let chrono_fmt = fmt.replace("{day_ordinal}", placeholder);

        return datetime
            .format(&chrono_fmt)
            .to_string()
            .replace(placeholder, &day_ordinal);
    }

    format!(
        "{} {}, {} at {}",
        datetime.format("%B"),
        day_ordinal,
        datetime.year(),
        format_ampm_time(&datetime),
    )
}

fn ordinal_suffix(day: u32) -> &'static str {
    match day {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

fn format_ampm_time<Tz: chrono::TimeZone>(datetime: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    datetime
        .format("%I:%M %p")
        .to_string()
        .trim_start_matches('0')
        .to_string()
}

pub fn days_since_epoch(epoch_millis: u128) -> u64 {
    // Convert milliseconds to seconds, then to days
    (epoch_millis / 1000 / 60 / 60 / 24) as u64
}
