use crate::app::{
    AppData,
    ui::settings::{general_settings_ui::general_settings_ui, log_settings_ui::log_settings_ui},
};
use eframe::egui::Ui;

pub fn settings_center_layout(app: &mut AppData, ui: &mut Ui) {
    if app.runtime.menu_state == 0 {
        general_settings_ui(app, ui)
    } else if app.runtime.menu_state == 1 {
        log_settings_ui(app, ui);
    }
}
