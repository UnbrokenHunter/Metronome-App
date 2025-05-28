use super::plot::draw_plot;
use crate::app::GrowthType;
use crate::app::MyApp;
use eframe::egui::{self, Ui};
use functions::calculate;

mod functions;

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.label("Practice:");
    ui.separator();

    ui.add(egui::Slider::new(&mut app.tempo_params.length, 0..=600).text("Practice Length"));

    ui.label("Tempo:");
    ui.separator();

    ui.add(egui::Slider::new(&mut app.tempo_params.min, 0..=app.tempo_params.max).text("Min"));
    ui.add(egui::Slider::new(&mut app.tempo_params.max, app.tempo_params.min..=120).text("Max"));

    ui.label("Growth Behavior:");
    ui.separator();

    ui.selectable_value(&mut app.growth_type, GrowthType::Linear, "Linear");
    ui.selectable_value(&mut app.growth_type, GrowthType::Sigmoidal, "Sigmoidal");
    ui.selectable_value(&mut app.growth_type, GrowthType::Logarithmic, "Logarithmic");
    ui.selectable_value(&mut app.growth_type, GrowthType::Exponential, "Exponential");
    ui.selectable_value(&mut app.growth_type, GrowthType::Constant, "Constant");
}

pub fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Tempo");

        if app.playing {
            app.time += 0.1;
            app.tempo = calculate(app.growth_type, app.time, app.tempo_params);
            app.points.push([app.time, app.tempo]);
        }

        draw_plot(ui, &app.points);

        if ui.add(egui::Button::new("Play")).clicked() {
            app.playing = !app.playing
        }
    });
}
