use crate::app::MyApp;
use eframe::egui::{self, Ui};

pub fn practice_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Practice:");
        ui.separator();
        ui.add(egui::Slider::new(&mut app.tempo_params.length, 1..=600).text("Practice Length"));
    });
}
