use eframe::egui::{Context, Key};

use crate::app::AppData;

pub fn check_keyboard(app: &mut AppData, ctx: Context) {
    ctx.input(|i| {
        // Manual Adjustment
        let manual_increment = 5.0;
        if i.key_pressed(Key::ArrowUp) {
            app.parameters.tempo_params.manual_offset += manual_increment;
        }
        if i.key_pressed(Key::ArrowDown) {
            if app.runtime.tempo - manual_increment >= 0.0 {
                app.parameters.tempo_params.manual_offset -= manual_increment;
            }
        }

        // Manual Time Adjustment
        let manual_time_increment = 5.0;
        if i.key_pressed(Key::ArrowRight) {
            app.parameters.tempo_params.manual_time_offset += manual_time_increment;
        }
        if i.key_pressed(Key::ArrowLeft) {
            let new_offset = app.parameters.tempo_params.manual_time_offset - manual_time_increment;
            let adjusted_time = app.runtime.time_data.calculated_time_since_start as i128
                + (new_offset as i128 * 1000);
            println!(
                "Test {} -- {}",
                app.runtime.time_data.calculated_time_since_start,
                new_offset as i128 * 1000
            );

            if adjusted_time > 0 {
                println!("Test {}", app.parameters.tempo_params.manual_time_offset);
                app.parameters.tempo_params.manual_time_offset = new_offset;
            }
        }

        if i.key_pressed(Key::E) {
            app.parameters.volume = f32::clamp(app.parameters.volume + 0.10, 0.0, 1.0)
        }

        if i.key_pressed(Key::Q) {
            app.parameters.volume = f32::clamp(app.parameters.volume - 0.10, 0.0, 1.0)
        }

        // Reset
        if i.key_pressed(Key::R) {
            app.reset_metronome();
        }

        // Max Tempo
        if i.key_pressed(Key::W) {
            app.parameters.tempo_params.max += 5
        }
        if i.key_pressed(Key::S) {
            app.parameters.tempo_params.max -= 5
        }
        // Min Tempo
        if i.key_pressed(Key::A) {
            app.parameters.tempo_params.min -= 5
        }
        if i.key_pressed(Key::D) {
            app.parameters.tempo_params.min += 5
        }
        // Space
        if i.key_pressed(Key::Space) {
            app.runtime.playing = !app.runtime.playing
        }
    });
}
