use super::plot::draw_demo_plot;
use super::plot::draw_plot;
use crate::app::GrowthType;
use crate::app::MyApp;
use eframe::egui::{self, Ui};
use functions::calculate;
use sound::play_metronome;

pub mod functions;
pub mod sound;

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Practice:");
        ui.separator();

        ui.add(egui::Slider::new(&mut app.tempo_params.length, 1..=600).text("Practice Length"));
    });

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Tempo:");
        ui.separator();

        if app.growth_type == GrowthType::Constant {
            ui.add(
                egui::Slider::new(&mut app.tempo_params.min, 0..=app.tempo_params.max)
                    .text("Tempo"),
            );
        } else {
            ui.add(
                egui::Slider::new(&mut app.tempo_params.min, 0..=app.tempo_params.max).text("Min"),
            );
            ui.add(
                egui::Slider::new(&mut app.tempo_params.max, app.tempo_params.min..=120)
                    .text("Max"),
            );
        }
    });

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Growth Behavior:");
        ui.separator();

        ui.horizontal(|ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut app.growth_type, GrowthType::Linear, "Linear");
                if ui
                    .selectable_value(&mut app.growth_type, GrowthType::Exponential, "Exponential")
                    .clicked()
                {
                    app.tempo_params.scaler = 3.0;
                }

                if ui
                    .selectable_value(&mut app.growth_type, GrowthType::Sigmoidal, "Sigmoidal")
                    .clicked()
                {
                    app.tempo_params.scaler = 6.0;
                }

                if ui
                    .selectable_value(&mut app.growth_type, GrowthType::Logarithmic, "Logarithmic")
                    .clicked()
                {
                    app.tempo_params.scaler = 0.5;
                }
                ui.selectable_value(&mut app.growth_type, GrowthType::Constant, "Constant");
            });

            ui.vertical(|ui| {
                // Display Scaler if type requires it
                if app.growth_type == GrowthType::Exponential {
                    ui.add(
                        egui::Slider::new(&mut app.tempo_params.scaler, 1.0..=10.0).text("Scaler"),
                    );
                } else if app.growth_type == GrowthType::Logarithmic {
                    ui.add(
                        egui::Slider::new(&mut app.tempo_params.scaler, 0.0..=1.0).text("Scaler"),
                    );
                } else if app.growth_type == GrowthType::Sigmoidal {
                    ui.add(
                        egui::Slider::new(&mut app.tempo_params.scaler, 1.0..=10.0).text("Scaler"),
                    );
                }

                draw_demo_plot(ui, app.growth_type, app.tempo_params);
            });
        });
    });

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Sounds:");
        ui.separator();

        ui.vertical(|ui| {
            ui.horizontal(|ui: &mut Ui| {
                ui.selectable_value(&mut app.sound, super::Sounds::Cowbell, "Cowbell");
                ui.selectable_value(&mut app.sound, super::Sounds::Thump, "Thump");
                ui.selectable_value(&mut app.sound, super::Sounds::Tone, "Tone");
            });
            ui.horizontal(|ui: &mut Ui| {
                ui.selectable_value(&mut app.sound, super::Sounds::Beep, "Beep");
                ui.selectable_value(&mut app.sound, super::Sounds::Clave, "Clave");
                ui.selectable_value(&mut app.sound, super::Sounds::Click, "Click");
            });
        });

        if ui.add(egui::Button::new("Play")).clicked() {
            println!("Button Click");
            play_metronome(app.sound);
        }
    });
}

pub fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Tempo");

        if app.playing {
            app.time += 0.1;
            app.tempo = calculate(app.growth_type, app.time, app.tempo_params);
            app.points.push([app.time, app.tempo]);
        }

        draw_plot(ui, &app.points, app.tempo_params);

        if ui.add(egui::Button::new("Play")).clicked() {
            app.playing = !app.playing
        }
    });
}
