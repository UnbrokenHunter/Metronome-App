use eframe::egui::{self, Ui};

use crate::app::{AppData, logic::logs};

pub fn play_ui(app: &mut AppData, ui: &mut Ui) {
    let size = [ui.available_width(), 30.0];

    ui.vertical(|ui| {
        if ui.add_sized(size, egui::Button::new("Play")).clicked() {
            app.runtime.playing = !app.runtime.playing;
        }
        if ui.add_sized(size, egui::Button::new("Reset")).clicked() {
            logs::try_add_log(app);
            app.reset_metronome();
        }
    });
}
