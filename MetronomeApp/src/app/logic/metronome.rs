use crate::app::{AppData, logic::sound::play_metronome, types::TimeData};

pub fn update_metronome(app: &mut AppData) {
    let tempo_seconds = app.runtime.tempo / 60.0;
    let period = 1.0 / tempo_seconds;

    let time_since_last_click: f64 = (app.runtime.time_data.calculated_time_since_start
        - (app.runtime.last_click_time)) as f64
        / 1000.0;

    if time_since_last_click > period {
        click(&mut app.runtime.last_click_time, app.runtime.time_data); // ← pass as mutable reference
        play_metronome(app, app.parameters.sound)
    }
}

fn click(last_click_time: &mut u128, time_data: TimeData) {
    *last_click_time = time_data.calculated_time_since_start;
}
