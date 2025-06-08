use crate::app::{AppData, logic};
use eframe::egui::{self, Context, ScrollArea};

pub fn logs_panel_layout(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("logs_panel")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    for (i, log) in app.practice.logs.iter().enumerate() {
                        // First draw the content inside a frame
                        let response = egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.allocate_ui_with_layout(
                                egui::vec2(ui.available_width(), 0.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.heading(format!(
                                        "{}",
                                        logic::clock::format_date(log.time_started, None)
                                    ));
                                    ui.separator();
                                    ui.label(format!(
                                        "Duration: {}",
                                        logic::clock::format_time(log.duration_ms as u128)
                                    ));
                                    ui.label(format!("Average Tempo: {}", log.average_tempo));
                                    ui.label(format!("Average Delta: {}", log.average_delta));
                                },
                            );
                        });

                        // Make the whole frame area clickable
                        let clicked = ui
                            .interact(
                                response.response.rect,
                                egui::Id::new(format!("log_{}", i)),
                                egui::Sense::click(),
                            )
                            .clicked();

                        if clicked {
                            println!("Clicked log at index {i}");
                            app.runtime.menu_state = i as u32;
                        }

                        ui.add_space(8.0); // Add spacing between logs
                    }
                });
            });
        });
}
