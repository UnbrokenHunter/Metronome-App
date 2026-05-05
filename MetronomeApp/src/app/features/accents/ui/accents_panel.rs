use crate::app::AppData;
use eframe::egui::{Frame, ScrollArea, Ui};

use super::panel::{draw_accent, draw_accent_top_bar};

pub fn accents_panel(app: &mut AppData, ui: &mut Ui) {
    Frame::group(ui.style()).show(ui, |ui| {
        draw_accent_top_bar(app, ui);

        ui.separator();

        Frame::group(ui.style()).show(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let total_width = ui.available_width();
                    let mut accent_index = 0;

                    while accent_index < app.parameters.accents.accents.len() {
                        draw_accent(app, ui, accent_index, total_width);
                        accent_index += 1;
                    }
                });
            });
        });
    });
}