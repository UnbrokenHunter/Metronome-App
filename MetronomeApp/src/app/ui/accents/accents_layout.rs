use crate::app::AppData;
use eframe::egui::Ui;

pub fn accents_layout(app: &mut AppData, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.label("Text");
    });
}
