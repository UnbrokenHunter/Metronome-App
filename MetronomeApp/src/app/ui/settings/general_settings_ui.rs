use eframe::egui::{self, RichText, ScrollArea, Ui};

use crate::app::AppData;

pub fn general_settings_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui.label(RichText::new("Settings").size(45.0));
            ui.separator();

            ui.heading("Reset All Settings");
            if ui
                .add_sized([ui.available_width(), 30.0], egui::Button::new("Reset"))
                .clicked()
            {
                app.reset_settings();
            }
        });
    });
}
