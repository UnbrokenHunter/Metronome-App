use crate::app::AppData;
use eframe::egui::{self, Ui};

use super::section::section;

pub fn practice(app: &mut AppData, ui: &mut Ui) {
    section(ui, "Exercise Length:", |ui| {
        ui.horizontal(|ui| {
            ui.add(
                egui::Slider::new(&mut app.parameters.tempo_params.length, 0.1..=60.0)
                    .text("Minutes"),
            );

            ui.add_space(15.0);

            ui.add(egui::Checkbox::new(
                &mut app.parameters.infinite,
                "Infinite",
            ));
        });
    });
}