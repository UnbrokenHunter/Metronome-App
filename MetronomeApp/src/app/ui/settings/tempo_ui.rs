use crate::app::{GrowthType, MyApp};
use eframe::egui::{self, Ui};

pub fn tempo_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Tempo:");
        ui.separator();

        if app.growth_type == GrowthType::Constant {
            ui.add(
                egui::Slider::new(&mut app.tempo_params.min, 0..=app.tempo_params.max)
                    .text("Tempo"),
            );
        } else {
            ui.add(
                egui::Slider::new(&mut app.tempo_params.min, 0..=app.tempo_params.max).text("Min"),
            );
            ui.add(
                egui::Slider::new(&mut app.tempo_params.max, app.tempo_params.min..=120)
                    .text("Max"),
            );
        }
    });
}
