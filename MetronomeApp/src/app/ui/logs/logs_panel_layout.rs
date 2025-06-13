use std::collections::BTreeMap;

use crate::app::{AppData, logic, types::PracticeLog};
use eframe::egui::{self, Context, ScrollArea};

pub fn logs_panel_layout(app: &mut AppData, ctx: &Context) {
    let mut grouped: BTreeMap<u64, Vec<(usize, &PracticeLog)>> = BTreeMap::new();
    for (i, log) in app.practice.logs.iter().enumerate() {
        let day = logic::clock::days_since_epoch(log.time_started);
        grouped.entry(day).or_default().push((i, log));
    }

    egui::SidePanel::left("logs_panel")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui: &mut egui::Ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(200.0, ui.available_height()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            for (i, group) in grouped.iter().enumerate() {
                                let date_label = format!(
                                    "{}",
                                    logic::clock::format_date(group.1[0].1.time_started, None)
                                );
                                egui::CollapsingHeader::new(date_label)
                                    .default_open(false) // optional
                                    .id_salt(format!("{}_collapse", i))
                                    .show(ui, |ui| {
                                        for (_j, log) in group.1.iter().enumerate() {
                                            // First draw the content inside a frame
                                            let response =
                                                egui::Frame::group(ui.style()).show(ui, |ui| {
                                                    ui.allocate_ui_with_layout(
                                                        egui::vec2(ui.available_width(), 0.0),
                                                        egui::Layout::top_down(egui::Align::Min),
                                                        |ui| {
                                                            let header = if log.1.title.is_empty() {
                                                                format!(
                                                                    "{}",
                                                                    logic::clock::format_date(
                                                                        log.1.time_started,
                                                                        None
                                                                    )
                                                                )
                                                            } else {
                                                                log.1.title.clone()
                                                            };
                                                            ui.heading(header);

                                                            ui.separator();
                                                            ui.label(format!(
                                                                "Duration: {}",
                                                                logic::clock::format_time(
                                                                    log.1.duration_ms as u128
                                                                )
                                                            ));

                                                            if log.1.notes.is_empty() {
                                                                ui.label(format!(
                                                                    "Average Tempo: {}",
                                                                    log.1.average_tempo
                                                                ));
                                                                ui.label(format!(
                                                                    "Average Delta: {}",
                                                                    log.1.average_delta
                                                                ));
                                                            } else {
                                                                ui.label(format!(
                                                                    "Notes: {}",
                                                                    log.1.notes
                                                                ));
                                                            }
                                                        },
                                                    );
                                                });

                                            // Make the whole frame area clickable
                                            let clicked = ui
                                                .interact(
                                                    response.response.rect,
                                                    egui::Id::new(format!("log_{}", log.0)),
                                                    egui::Sense::click(),
                                                )
                                                .clicked();

                                            if clicked {
                                                println!("Clicked log at index {}", log.0);
                                                app.runtime.menu_state = log.0 as u32;
                                            }

                                            ui.add_space(8.0); // Add spacing between logs
                                        }
                                    });
                            }
                        });
                    },
                );
            });
        });
}
