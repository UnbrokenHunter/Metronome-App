use eframe::egui::{Context, Key};

use crate::app::MyApp;

pub fn check_keyboard(app: &mut MyApp, ctx: Context) {
    ctx.input(|i| {
        // Manual Adjustment
        let manual_increment = 5.0;
        if i.key_pressed(Key::ArrowUp) {
            app.tempo_params.manual_offset += manual_increment;
        }
        if i.key_pressed(Key::ArrowDown) {
            if app.tempo - manual_increment >= 0.0 {
                app.tempo_params.manual_offset -= manual_increment;
            }
        }

        // Manual Time Adjustment
        let manual_time_increment = 5.0;
        if i.key_pressed(Key::ArrowRight) {
            app.tempo_params.manual_time_offset += manual_time_increment;
        }
        if i.key_pressed(Key::ArrowLeft) {
            let new_offset = app.tempo_params.manual_time_offset - manual_time_increment;
            let adjusted_time =
                app.time_data.calculated_time_since_start as i128 + (new_offset as i128 * 1000);
            println!(
                "Test {} -- {}",
                app.time_data.calculated_time_since_start,
                new_offset as i128 * 1000
            );

            if adjusted_time > 0 {
                println!("Test {}", app.tempo_params.manual_time_offset);
                app.tempo_params.manual_time_offset = new_offset;
            }
        }

        // Reset
        if i.key_pressed(Key::R) {
            app.reset_metronome();
        }

        // Max Tempo
        if i.key_pressed(Key::W) {
            app.tempo_params.max += 5
        }
        if i.key_pressed(Key::S) {
            app.tempo_params.max -= 5
        }
        // Min Tempo
        if i.key_pressed(Key::A) {
            app.tempo_params.min -= 5
        }
        if i.key_pressed(Key::D) {
            app.tempo_params.min += 5
        }
        // Space
        if i.key_pressed(Key::Space) {
            app.playing = !app.playing
        }
    });
}
