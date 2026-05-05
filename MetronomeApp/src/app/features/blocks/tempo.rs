use crate::app::{AppData, GrowthType};
use eframe::egui::{self, Ui};

use super::section::section;

pub fn tempo(app: &mut AppData, ui: &mut Ui) {
    section(ui, "Tempo:", |ui| {
        if app.parameters.growth_type == GrowthType::Constant {
            tempo_row(app, ui, "Tempo", TempoTarget::Min);
        } else {
            tempo_row(app, ui, "Starting Tempo  ", TempoTarget::Min);
            tempo_row(app, ui, "Ending Tempo    ", TempoTarget::Max);
        }
    });
}

#[derive(Clone, Copy)]
enum TempoTarget {
    Min,
    Max,
}

fn tempo_row(app: &mut AppData, ui: &mut Ui, label: &str, target: TempoTarget) {
    ui.horizontal(|ui| {
        match target {
            TempoTarget::Min => {
                ui.add(
                    egui::Slider::new(&mut app.parameters.tempo_params.min, 1..=400)
                        .text(label),
                );

                if ui.add(egui::Button::new("Tap")).clicked() {
                    app.parameters.tempo_params.min = tap_tempo(app);
                }
            }

            TempoTarget::Max => {
                ui.add(
                    egui::Slider::new(&mut app.parameters.tempo_params.max, 1..=400)
                        .text(label),
                );

                if ui.add(egui::Button::new("Tap")).clicked() {
                    app.parameters.tempo_params.max = tap_tempo(app);
                }
            }
        }
    });
}

fn tap_tempo(app: &mut AppData) -> u32 {
    let last = app.runtime.last_tap_tempo_click;
    let now = app.runtime.time_data.time_since_start;

    app.runtime.last_tap_tempo_click = now;

    if last == 0 {
        return app.runtime.tempo as u32;
    }

    let elapsed_seconds = (now - last) as f64 / 1000.0;

    (60.0 / elapsed_seconds) as u32
}