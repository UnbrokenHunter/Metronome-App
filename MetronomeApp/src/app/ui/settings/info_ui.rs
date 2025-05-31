use crate::app::logic::functions::calculate;
use crate::app::{MyApp, logic::functions::derivative};
use eframe::egui::{self, Frame, Grid, RichText, Ui};

pub fn info_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        Frame::group(ui.style())
            .inner_margin(crate::app::ui::settings::info_ui::egui::Margin::same(4)) // default is often ~6.0
            .outer_margin(crate::app::ui::settings::info_ui::egui::Margin::same(0))
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

                        let total_seconds = app.time_data.calculated_time_since_start / 1000;
                        let hours = total_seconds / 3600;
                        let minutes = (total_seconds % 3600) / 60;
                        let seconds =
                            (app.time_data.calculated_time_since_start as f64 / 1000.0) % 60.0;

                        ui.label(RichText::new("Time:").size(28.0));
                        ui.label(
                            RichText::new(format!("{:02}:{:02}:{:05.2}", hours, minutes, seconds))
                                .size(23.0),
                        );
                        ui.end_row();

                        // Row 3: Delta
                        let f = |x: f64| calculate(app.growth_type, x, app.tempo_params);

                        ui.label(RichText::new("ΔBPM:").size(28.0));
                        ui.label(
                            RichText::new(format!(
                                "{:.2} BPM/s",
                                derivative(
                                    f,
                                    app.time_data.calculated_time_since_start as f64 / 1000.0
                                )
                            ))
                            .size(23.0),
                        );
                        ui.end_row();
                    });
            });
    });
}
