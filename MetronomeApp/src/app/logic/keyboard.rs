use eframe::egui::{Context, Key};

use crate::app::MyApp;

pub fn check_keyboard(app: &mut MyApp, ctx: Context) {
    ctx.input(|i| {
        // Manual Adjustment
        if i.key_pressed(Key::ArrowUp) {
            app.tempo_params.manual_offset += 5.0;
        }
        if i.key_pressed(Key::ArrowDown) {
            if app.tempo - 5.0 >= 0.0 {
                app.tempo_params.manual_offset -= 5.0;
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
