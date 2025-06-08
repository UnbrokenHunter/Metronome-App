use eframe::egui::{self, Ui};

use crate::app::AppData;

pub fn play_ui(app: &mut AppData, ui: &mut Ui) {
    let half_width: f32 = ui.available_width() / 2.0;

    ui.vertical(|ui| {
        if ui
            .add_sized([half_width, 30.0], egui::Button::new("Play"))
            .clicked()
        {
            app.runtime.playing = !app.runtime.playing;
        }
        if ui
            .add_sized([half_width, 30.0], egui::Button::new("Reset"))
            .clicked()
        {
            app.try_add_log(
                app.runtime.time_data.calculated_time_since_start as u64,
                app.save.tempo_params.min,
                app.save.tempo_params.max,
            );
            app.reset_metronome();
        }
        if ui
            .add_sized([half_width, 30.0], egui::Button::new("Revert Defaults"))
            .clicked()
        {
            app.try_add_log(
                app.runtime.time_data.calculated_time_since_start as u64,
                app.save.tempo_params.min,
                app.save.tempo_params.max,
            );
            app.reset_all_settings();
        }
    });
}
