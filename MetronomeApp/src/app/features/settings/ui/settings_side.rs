use crate::app::AppData;
use eframe::egui::{self, Ui};

pub fn settings_side(app: &mut AppData, ui: &mut Ui) {
    settings_nav_button(app, ui, 0, "General");
    settings_nav_button(app, ui, 1, "Logs");
}

fn settings_nav_button(app: &mut AppData, ui: &mut Ui, menu_state: u32, label: &str) {
    let selected = app.runtime.menu_state == menu_state;

    if ui
        .add_sized(
            [ui.available_width(), 30.0],
            egui::Button::new(label).selected(selected),
        )
        .clicked()
    {
        app.runtime.menu_state = menu_state;
    }
}
