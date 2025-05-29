use crate::app::MyApp;
use eframe::egui::{self, Ui};

pub fn practice_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Excercise Length:");
        ui.separator();
        ui.add(egui::Slider::new(&mut app.tempo_params.length, 0.1..=20.0).text("Minutes"));
    });
}
