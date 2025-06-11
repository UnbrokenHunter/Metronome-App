use crate::app::{
    AppData,
    logic::{
        accents::{calculate_number_of_beats, get_accent_at_beat_index, get_beat_at_index},
        sound::play_metronome,
    },
    types::BeatState,
};

pub fn update_metronome(app: &mut AppData) {
    let tempo_seconds = app.runtime.tempo / 60.0;
    let period = 1.0 / tempo_seconds;

    let time_since_last_click: f64 = (app.runtime.time_data.calculated_time_since_start
        - (app.runtime.last_click_time)) as f64
        / 1000.0;

    let subdivision_period = period
        / get_accent_at_beat_index(app, app.runtime.last_click_accent as usize)
            .unwrap()
            .subdivision as f64;
    let time_since_last_subdivision: f64 = (app.runtime.time_data.calculated_time_since_start
        - (app.runtime.last_subdivision_time)) as f64
        / 1000.0;

    if time_since_last_click > period {
        app.runtime.last_click_time = app.runtime.time_data.calculated_time_since_start;
        app.runtime.last_subdivision_time = app.runtime.time_data.calculated_time_since_start;

        app.runtime.last_click_accent += 1;
        if app.runtime.last_click_accent >= calculate_number_of_beats(app) as u32 {
            app.runtime.last_click_accent = 0;
        }
        // println!("Accent: {}", app.runtime.last_click_accent);

        let sound: String = app.parameters.sound.to_string().to_lowercase();

        let beat: &mut crate::app::types::BeatData =
            get_beat_at_index(app, app.runtime.last_click_accent as usize).unwrap();

        if beat.state == BeatState::Downbeat {
            play_metronome(app, format!("{}/{}", sound, sound));
        } else if beat.state == BeatState::Strong {
            play_metronome(app, format!("{}/{}_strong", sound, sound));
        } else if beat.state == BeatState::Weak {
            play_metronome(app, format!("{}/{}_weak", sound, sound));
        }
    } else if time_since_last_subdivision > subdivision_period {
        // Only do the subdivision if not a main beat
        app.runtime.last_subdivision_time = app.runtime.time_data.calculated_time_since_start;
        let sound: String = app.parameters.sound.to_string().to_lowercase();
        play_metronome(app, format!("{}/{}_subdivision", sound, sound));
    }
}
