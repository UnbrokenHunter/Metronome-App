use crate::app::{AppData, ui::settings::log_settings_ui::log_settings_ui};
use eframe::egui::{self, Ui};

pub fn settings_panel_layout(app: &mut AppData, ui: &mut Ui) {
    // ui.vertical_centered(|ui| {
    if app.runtime.menu_state == 0 {
        log_settings_ui(app, ui);
    } else if app.runtime.menu_state == 1 {
        egui::Frame::group(ui.style()).show(ui, |ui| ui.label("Menu State 1"));
    }
    // });
    
}
