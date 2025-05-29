use crate::app::logic::functions::calculate;
use crate::app::ui::plot::draw_plot;
use crate::app::ui::settings;
use crate::app::{MyApp, logic::functions::derivative};
use eframe::egui::{self, Frame, Grid, RichText, Ui};

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    settings::practice_ui(app, ui);
    settings::tempo_ui(app, ui);

    settings::growth_ui(app, ui);
    settings::sound_ui(app, ui);
}

pub fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        if app.playing {
            app.tempo = calculate(
                app.growth_type,
                app.time_data.calculated_time_since_start as f64 / 1000.0,
                app.tempo_params,
            );
            // Clamp Values
            if app.tempo > app.tempo_params.max as f64 {
                app.tempo = app.tempo_params.max as f64;
            }
            if app.tempo < app.tempo_params.min as f64 {
                app.tempo = app.tempo_params.min as f64;
            }

            app.points.push([
                app.time_data.calculated_time_since_start as f64 / 1000.0,
                app.tempo,
            ]);
        }

        egui::Frame::group(ui.style()).show(ui, |ui| {
            draw_plot(ui, &app.points, app.tempo_params);
        });

        ui.horizontal(|ui: &mut Ui| {
            let half_width = ui.available_width() / 2.0;

            ui.vertical(|ui| {
                if ui
                    .add_sized([half_width, 30.0], egui::Button::new("Play"))
                    .clicked()
                {
                    app.playing = !app.playing;
                }
                if ui
                    .add_sized([half_width, 30.0], egui::Button::new("Reset"))
                    .clicked()
                {
                    app.reset_metronome();
                }
                if ui
                    .add_sized([half_width, 30.0], egui::Button::new("Revert Defaults"))
                    .clicked()
                {
                    app.reset_all_settings();
                }
            });

            ui.vertical(|ui| {
                Frame::group(ui.style())
                    .inner_margin(crate::app::ui::layout::egui::Margin::same(4)) // default is often ~6.0
                    .outer_margin(crate::app::ui::layout::egui::Margin::same(0))
                    .show(ui, |ui| {
                        Grid::new("tempo_time_grid")
                            .num_columns(2)
                            .spacing(egui::vec2(20.0, 10.0))
                            .striped(true)
                            .show(ui, |ui| {
                                // Row 1: Tempo
                                ui.label(RichText::new("BPM:").size(28.0));
                                ui.label(RichText::new(format!("{:.2} BPM", app.tempo)).size(23.0));
                                ui.end_row();

                                let total_seconds =
                                    app.time_data.calculated_time_since_start / 1000;
                                let hours = total_seconds / 3600;
                                let minutes = (total_seconds % 3600) / 60;
                                let seconds = (app.time_data.calculated_time_since_start as f64
                                    / 1000.0)
                                    % 60.0;

                                ui.label(RichText::new("Time:").size(28.0));
                                ui.label(
                                    RichText::new(format!(
                                        "{:02}:{:02}:{:05.2}",
                                        hours, minutes, seconds
                                    ))
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
                                            app.time_data.calculated_time_since_start as f64
                                                / 1000.0
                                        )
                                    ))
                                    .size(23.0),
                                );
                                ui.end_row();
                            });
                    });
            });
        });
    });
}
