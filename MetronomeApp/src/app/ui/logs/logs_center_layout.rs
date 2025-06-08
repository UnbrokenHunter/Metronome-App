use crate::app::{
    AppData,
    logic::clock::{format_date, format_time},
    ui::graph::plot::draw_plot,
};
use eframe::egui::{self, ScrollArea, Ui};

pub fn logs_center_layout(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            if let Some(log) = app.practice.logs.get(app.runtime.menu_state as usize) {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.heading(format!("Date: {}", format_date(log.time_started, None)));
                    ui.separator();
                    draw_plot(ui, &log.points);
                    ui.separator();

                    ui.heading(format!(
                        "Duration: {}",
                        format_time(log.duration_ms as u128)
                    ));
                    ui.label(format!("Average Tempo: {}", log.average_tempo));
                    ui.label(format!("Average Delta: {}", log.average_delta));

                    ui.label(format!("Starting Tempo: {}", log.min_tempo));
                    ui.label(format!("Ending Tempo: {}", log.max_tempo));
                });
            }
        });
    });
}
