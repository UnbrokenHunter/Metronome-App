use crate::app::{AppData, GrowthType};
use eframe::egui::{self, Ui};

pub fn tempo_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Tempo:");
        ui.separator();

        if app.save.growth_type == GrowthType::Constant {
            ui.add(egui::Slider::new(&mut app.save.tempo_params.min, 1..=400).text("Tempo"));
        } else {
            ui.add(
                egui::Slider::new(&mut app.save.tempo_params.min, 1..=400).text("Starting Tempo"),
            );
            ui.add(egui::Slider::new(&mut app.save.tempo_params.max, 1..=400).text("Ending Tempo"));
        }
    });
}
