use crate::app::systems::audio;
use crate::app::{
    logic::accents::{calculate_number_of_beats, get_accent_at_beat_index, get_beat_at_index}, AppData,
    BeatState,
};

pub fn update_metronome(app: &mut AppData) {
    let total_beats = calculate_number_of_beats(app) as u32;

    // Nothing to play.
    if total_beats == 0 || app.runtime.tempo <= 0.0 {
        app.runtime.last_click_accent = 0;
        return;
    }

    // Keep last_click_accent valid.
    if app.runtime.last_click_accent >= total_beats {
        app.runtime.last_click_accent = 0;
    }

    let period = 60.0 / app.runtime.tempo;

    let now = app.runtime.time_data.calculated_time_since_start;

    let time_since_last_click = now.saturating_sub(app.runtime.last_click_time) as f64 / 1000.0;

    let Some(current_accent) =
        get_accent_at_beat_index(app, app.runtime.last_click_accent as usize)
    else {
        app.runtime.last_click_accent = 0;
        return;
    };

    let subdivision = current_accent.subdivision.max(1);
    let subdivision_period = period / subdivision as f64;

    let time_since_last_subdivision =
        now.saturating_sub(app.runtime.last_subdivision_time) as f64 / 1000.0;

    if time_since_last_click >= period {
        app.runtime.last_click_time = now;
        app.runtime.last_subdivision_time = now;

        play_current_beat(app);

        app.runtime.last_click_accent += 1;

        if app.runtime.last_click_accent >= total_beats {
            app.runtime.last_click_accent = 0;
        }
    } else if time_since_last_subdivision > subdivision_period {
        app.runtime.last_subdivision_time = now;

        let sound = app.parameters.sound.to_string().to_lowercase();
        let volume = app.parameters.volume;
        audio::play_audio_from_file(&format!("{sound}/{sound}_subdivision"), volume);
    }
}

fn play_current_beat(app: &mut AppData) {
    let sound = app.parameters.sound.to_string().to_lowercase();
    let volume = app.parameters.volume;

    let Some(beat) = get_beat_at_index(app, app.runtime.last_click_accent as usize) else {
        app.runtime.last_click_accent = 0;
        return;
    };

    match beat.state {
        BeatState::Downbeat => {
            audio::play_audio_from_file(&format!("{sound}/{sound}"), volume);
        }
        BeatState::Strong => {
            audio::play_audio_from_file(&format!("{sound}/{sound}_strong"), volume);
        }
        BeatState::Weak => {
            audio::play_audio_from_file(&format!("{sound}/{sound}_weak"), volume);
        }
        BeatState::Off => {
            // No sound for off beats.
        }
    }
}
