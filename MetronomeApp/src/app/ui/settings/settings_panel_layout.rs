use crate::app::{AppData, ui::settings::log_settings_ui::log_settings_ui};
use eframe::egui::Ui;

pub fn settings_panel_layout(app: &mut AppData, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        log_settings_ui(app, ui);
    });
}
