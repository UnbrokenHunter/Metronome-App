use eframe::egui::{self, Ui};

use crate::app::AppData;

pub fn log_settings_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading("Log Settings");
        ui.separator();

        ui.label("Delete Logs");
        if ui
            .add_sized([ui.available_width(), 30.0], egui::Button::new("Trash"))
            .clicked()
        {}
    });
}
