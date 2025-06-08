use crate::app::logic::clock::format_time;
use crate::app::logic::functions::calculate;
use crate::app::{AppData, logic::functions::derivative};
use eframe::egui::{self, Frame, Grid, RichText, Ui};

pub fn info_ui(app: &mut AppData, ui: &mut Ui) {
    ui.vertical(|ui| {
        Frame::group(ui.style())
            .inner_margin(crate::app::ui::general::info_ui::egui::Margin::same(4)) // default is often ~6.0
            .outer_margin(crate::app::ui::general::info_ui::egui::Margin::same(0))
            .show(ui, |ui| {
                Grid::new("tempo_time_grid")
                    .num_columns(2)
                    .spacing(egui::vec2(20.0, 10.0))
                    .striped(true)
                    .show(ui, |ui| {
                        // Row 1: Tempo
                        // ui.label(RichText::new("BPM:").size(28.0));
                        // ui.label(RichText::new(format!("{:.2} BPM", app.tempo)).size(23.0));
                        // ui.end_row();

                        ui.label(
                            RichText::new(
                                if app.save.tempo_params.manual_time_offset.abs() < 1e-6 {
                                    "Time"
                                } else {
                                    "ET:"
                                },
                            )
                            .size(28.0),
                        );
                        ui.label(
                            RichText::new(format_time(
                                app.runtime.time_data.calculated_time_since_start,
                            ))
                            .size(28.0),
                        );
                        ui.end_row();

                        // Manual Time Offset Time
                        if app.save.tempo_params.manual_time_offset != 0.0 {
                            ui.label(RichText::new("MT:").size(28.0));
                            ui.label(
                                RichText::new(format_time(
                                    (app.runtime.time_data.calculated_time_since_start as f64
                                        + app.save.tempo_params.manual_time_offset * 1000.0)
                                        as u128,
                                ))
                                .size(28.0),
                            );
                            ui.end_row();
                        }

                        // Row 3: Delta
                        let f = |x: f64| calculate(app.save.growth_type, x, app.save.tempo_params);

                        ui.label(RichText::new("ΔBPM:").size(28.0));
                        ui.label(
                            RichText::new(format!(
                                "{:.2} BPM/s",
                                derivative(
                                    f,
                                    app.runtime.time_data.calculated_time_since_start as f64
                                        / 1000.0
                                )
                            ))
                            .size(28.0),
                        );
                        ui.end_row();
                    });
            });
    });
}
