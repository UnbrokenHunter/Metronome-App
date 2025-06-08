use eframe::egui::{self, Ui};

use crate::app::AppData;

pub fn log_settings_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| ui.label("Log Settings"));
}
