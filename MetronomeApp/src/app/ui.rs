use super::plot::draw_plot;
use crate::app::GrowthType;
use crate::app::MyApp;
use eframe::egui::{self, Ui};

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.label("Practice:");
    ui.separator();

    ui.add(egui::Slider::new(&mut app.practice_length, 0..=600).text("Practice Length"));

    ui.label("Tempo:");
    ui.separator();

    ui.add(egui::Slider::new(&mut app.minimum_tempo, 0..=app.maximum_tempo).text("Min"));
    ui.add(egui::Slider::new(&mut app.maximum_tempo, app.minimum_tempo..=120).text("Max"));

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
            app.tempo += 0.1;
            let y = app.tempo.sin();
            app.points.push([app.tempo, y]);
            if app.points.len() > 500 {
                app.points.remove(0);
            }
        }

        draw_plot(ui, &app.points);

        if ui.add(egui::Button::new("Play")).clicked() {
            app.playing = !app.playing
        }
    });
}
