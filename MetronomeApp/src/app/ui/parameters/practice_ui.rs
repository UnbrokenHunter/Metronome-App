use crate::app::AppData;
use eframe::egui::{self, Ui};

pub fn practice_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Excercise Length:");
        ui.separator();
        ui.horizontal(|ui| {
            ui.add(
                egui::Slider::new(&mut app.parameters.tempo_params.length, 0.1..=60.0).text("Minutes"),
            );
            ui.add_space(15.0);
            ui.add(egui::Checkbox::new(&mut app.parameters.infinte, "Infinite"));
        });
    });
}
