use crate::app::{logic, AppData};
use eframe::egui::{self, Ui};

pub fn status(app: &mut AppData, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label(format!(
                "Uptime: {}",
                logic::clock::format_time(app.runtime.time_data.time_since_start)
            ));
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.label(format!(
                "Manual Offset: {} BPM",
                app.parameters.tempo_params.manual_offset
            ));
        });
    });
}
