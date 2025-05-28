use crate::app::logic::sound::play_metronome;
use crate::app::{MyApp, Sounds};
use eframe::egui::{self, Ui};

pub fn sound_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Sounds:");
        ui.separator();

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut app.sound, Sounds::Cowbell, "Cowbell");
                ui.selectable_value(&mut app.sound, Sounds::Thump, "Thump");
                ui.selectable_value(&mut app.sound, Sounds::Tone, "Tone");
            });
            ui.horizontal(|ui| {
                ui.selectable_value(&mut app.sound, Sounds::Beep, "Beep");
                ui.selectable_value(&mut app.sound, Sounds::Clave, "Clave");
                ui.selectable_value(&mut app.sound, Sounds::Click, "Click");
            });
        });
    });
}
