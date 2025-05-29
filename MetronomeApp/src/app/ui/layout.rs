use crate::app::MyApp;
use crate::app::logic::functions::calculate;
use crate::app::ui::plot::draw_plot;
use crate::app::ui::settings;
use eframe::egui::{self, RichText, Ui};

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
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(RichText::new("Tempo").size(30.0));
                        ui.label(RichText::new(format!("{:.2} BPM", app.tempo)).size(25.0));
                    });
                });
            });
        });
    });
}
