use crate::app::{GrowthType, MyApp};
use eframe::egui::{self, Ui};

pub fn tempo_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Tempo:");
        ui.separator();

        if app.growth_type == GrowthType::Constant {
            ui.add(
                egui::Slider::new(&mut app.tempo_params.min, 1..=app.tempo_params.max)
                    .text("Tempo"),
            );
        } else {
            ui.add(egui::Slider::new(&mut app.tempo_params.min, 1..=400).text("Starting Tempo"));
            ui.add(egui::Slider::new(&mut app.tempo_params.max, 1..=400).text("Ending Tempo"));
        }
    });
}
