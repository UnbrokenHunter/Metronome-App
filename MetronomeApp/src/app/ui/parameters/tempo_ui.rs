use crate::app::{AppData, GrowthType};
use eframe::egui::{self, Ui};

pub fn tempo_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label("Tempo:");
        ui.separator();
        if app.parameters.growth_type == GrowthType::Constant {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut app.parameters.tempo_params.min, 1..=400).text("Tempo"),
                );
                if ui.add(egui::Button::new("Tap")).clicked() {
                    app.parameters.tempo_params.min = tap_tempo(app);
                }
            });
        } else {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut app.parameters.tempo_params.min, 1..=400)
                        .text("Starting Tempo  "),
                );
                if ui.add(egui::Button::new("Tap")).clicked() {
                    app.parameters.tempo_params.min = tap_tempo(app);
                }
            });
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut app.parameters.tempo_params.max, 1..=400)
                        .text("Ending Tempo    "),
                );
                if ui.add(egui::Button::new("Tap")).clicked() {
                    app.parameters.tempo_params.max = tap_tempo(app);
                }
            });
        }
    });
}

fn tap_tempo(app: &mut AppData) -> u32 {
    let last = app.runtime.last_tap_tempo_click;
    app.runtime.last_tap_tempo_click = app.runtime.time_data.time_since_start;
    if last != 0 {
        return (1.0 / ((app.runtime.last_tap_tempo_click - last) as f64 / 1000.0) as f64 * 60.0)
            as u32;
    } else {
        return app.runtime.tempo as u32;
    }
}
