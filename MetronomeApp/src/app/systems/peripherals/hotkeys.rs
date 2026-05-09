use eframe::egui::Key;

use crate::app::systems::peripherals::keyboard_input::Keyboard;
use crate::app::AppData;

pub fn check_keyboard(app: &mut AppData) {
    // Manual tempo adjustment
    let manual_increment = 5.0;

    if Keyboard::pressed(Key::ArrowUp) {
        app.parameters.tempo_params.manual_offset += manual_increment;
    }

    if Keyboard::pressed(Key::ArrowDown) && app.runtime.tempo - manual_increment >= 0.0 {
        app.parameters.tempo_params.manual_offset -= manual_increment;
    }

    // Manual time adjustment
    let manual_time_increment = 5.0;

    if Keyboard::pressed(Key::ArrowRight) {
        app.parameters.tempo_params.manual_time_offset += manual_time_increment;
    }

    if Keyboard::pressed(Key::ArrowLeft) {
        let new_offset = app.parameters.tempo_params.manual_time_offset - manual_time_increment;

        let adjusted_time =
            app.runtime.time_data.calculated_time_since_start as i128 + (new_offset as i128 * 1000);

        if adjusted_time > 0 {
            app.parameters.tempo_params.manual_time_offset = new_offset;
        }
    }

    // Volume
    if Keyboard::pressed(Key::E) {
        app.parameters.volume = f32::clamp(app.parameters.volume + 0.10, 0.0, 1.0);
    }

    if Keyboard::pressed(Key::Q) {
        app.parameters.volume = f32::clamp(app.parameters.volume - 0.10, 0.0, 1.0);
    }

    // Reset
    if Keyboard::pressed(Key::R) {
        app.reset_metronome();
    }

    // Max tempo
    if Keyboard::pressed(Key::W) {
        app.parameters.tempo_params.max += 5;
    }

    if Keyboard::pressed(Key::S) {
        app.parameters.tempo_params.max -= 5;
    }

    // Min tempo
    if Keyboard::pressed(Key::A) {
        app.parameters.tempo_params.min -= 5;
    }

    if Keyboard::pressed(Key::D) {
        app.parameters.tempo_params.min += 5;
    }

    // Play / pause
    if Keyboard::pressed(Key::Space) {
        app.runtime.playing = !app.runtime.playing;
    }
}
