use super::plot::draw_plot;
use crate::app::GrowthType;
use crate::app::MyApp;
use eframe::egui::{self, Ui};
use functions::calculate;

mod functions;

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Practice:");
        ui.separator();

        ui.add(egui::Slider::new(&mut app.tempo_params.length, 0..=600).text("Practice Length"));
    });

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Tempo:");
        ui.separator();

        ui.add(egui::Slider::new(&mut app.tempo_params.min, 0..=app.tempo_params.max).text("Min"));
        ui.add(
            egui::Slider::new(&mut app.tempo_params.max, app.tempo_params.min..=120).text("Max"),
        );
    });

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Growth Behavior:");
        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut app.growth_type, GrowthType::Linear, "Linear");
                ui.selectable_value(&mut app.growth_type, GrowthType::Sigmoidal, "Sigmoidal");
                ui.selectable_value(&mut app.growth_type, GrowthType::Logarithmic, "Logarithmic");
                ui.selectable_value(&mut app.growth_type, GrowthType::Exponential, "Exponential");
                ui.selectable_value(&mut app.growth_type, GrowthType::Constant, "Constant");
            });

            // Display Scaler if type requires it
            if app.growth_type == GrowthType::Exponential {
                ui.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(&mut app.tempo_params.scaler, 1.0..=10.0).text("Scaler"),
                    );
                });
            } else if app.growth_type == GrowthType::Logarithmic {
                ui.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(&mut app.tempo_params.scaler, 0.0..=1.0).text("Scaler"),
                    );
                });
            }
        });
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
