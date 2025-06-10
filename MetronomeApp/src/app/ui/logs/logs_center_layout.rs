use crate::app::{
    AppData,
    logic::clock::{format_date, format_time, weekday_from_unix_ms},
    ui::graph::plot::draw_plot,
};
use eframe::egui::{self, Color32, RichText, ScrollArea, TextEdit, TextStyle, Ui};

pub fn logs_center_layout(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            if let Some(log) = app.practice.logs.get_mut(app.runtime.menu_state as usize) {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "{}, {}",
                                weekday_from_unix_ms(log.time_started),
                                format_date(log.time_started, None)
                            ))
                            .size(28.0)
                            .strong(),
                        );
                    });
                    ui.separator();
                    ui.add_sized(
                        [ui.available_width(), 27.0],
                        TextEdit::singleline(&mut log.title)
                            .font(TextStyle::Heading)
                            .hint_text("Title..."),
                    );
                    ui.horizontal(|ui| {
                        let total_width = ui.available_width();
                        let plot_width = total_width * 2.0 / 3.0;
                        let label_width = total_width - plot_width;

                        // Plot area: 2/3 width
                        ui.allocate_ui_with_layout(
                            egui::vec2(plot_width, ui.available_height()),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                draw_plot(ui, &log.points);
                            },
                        );

                        // Label area: 1/3 width
                        ui.allocate_ui_with_layout(
                            egui::vec2(label_width, ui.available_height()),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                egui::Frame::group(ui.style()).show(ui, |ui| {
                                    ui.set_min_width(ui.available_width()); // 🔑 Same here
                                    ui.label(
                                        RichText::new("Info:").color(Color32::GRAY).size(20.0),
                                    );
                                    ui.separator();

                                    egui::Frame::group(ui.style()).show(ui, |ui| {
                                        ui.set_min_width(ui.available_width()); // 🔑 Force full-width inside the frame
                                        ui.heading("General");
                                        ui.separator();
                                        ui.label(format!(
                                            "Duration:  {}",
                                            format_time(log.duration_ms as u128)
                                        ));
                                    });

                                    ui.separator();

                                    egui::Frame::group(ui.style()).show(ui, |ui| {
                                        ui.set_min_width(ui.available_width()); // 🔑 Force full-width inside the frame
                                        ui.heading("Tempo");
                                        ui.separator();
                                        ui.label(format!("Starting Tempo:   {}", log.min_tempo));
                                        ui.label(format!("Ending Tempo:     {}", log.max_tempo));
                                    });

                                    ui.separator();

                                    egui::Frame::group(ui.style()).show(ui, |ui| {
                                        ui.set_min_width(ui.available_width()); // 🔑 Same here
                                        ui.heading("Statistics");
                                        ui.separator();
                                        ui.label(format!("Average Tempo:  {}", log.average_tempo));
                                        ui.label(format!(
                                            "Average Delta:     {}",
                                            log.average_delta
                                        ));
                                    });
                                });
                            },
                        );
                    });
                    ui.separator();

                    ui.add_sized(
                        [ui.available_width(), 100.0], // width = fill available; height = 100 px
                        egui::TextEdit::multiline(&mut log.notes).hint_text("Add Notes..."),
                    );
                });
            }
        });
    });
}
