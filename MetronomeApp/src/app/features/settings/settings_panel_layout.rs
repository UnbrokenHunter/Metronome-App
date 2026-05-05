use crate::app::AppData;
use eframe::egui::{self, Ui};

pub fn settings_panel_layout(app: &mut AppData, ui: &mut Ui) {
    if ui
        .add_sized([ui.available_width(), 30.0], egui::Button::new("General"))
        .clicked()
    {
        app.runtime.menu_state = 0;
    }
    // Open Settings Button
    if ui
        .add_sized([ui.available_width(), 30.0], egui::Button::new("Logs"))
        .clicked()
    {
        app.runtime.menu_state = 1;
    }
}
