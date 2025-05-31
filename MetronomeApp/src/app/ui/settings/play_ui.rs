use eframe::egui::{self, Ui};

use crate::app::MyApp;

pub fn play_ui(app: &mut MyApp, ui: &mut Ui) {
    let half_width: f32 = ui.available_width() / 2.0;

    ui.vertical(|ui| {
        if ui
            .add_sized([half_width, 30.0], egui::Button::new("Play"))
            .clicked()
        {
            app.playing = !app.playing;
        }
        if ui
            .add_sized([half_width, 30.0], egui::Button::new("Reset"))
            .clicked()
        {
            app.reset_metronome();
        }
        if ui
            .add_sized([half_width, 30.0], egui::Button::new("Revert Defaults"))
            .clicked()
        {
            app.reset_all_settings();
        }
    });
}
