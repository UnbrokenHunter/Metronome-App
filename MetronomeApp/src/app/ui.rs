use super::plot::draw_plot;
use crate::app::MyApp;
use eframe::egui::{self, Ui};

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.label("Tempo:");
    ui.separator();

    ui.add(egui::Slider::new(&mut app.minimum_tempo, 0..=app.maximum_tempo).text("Min"));
    ui.add(egui::Slider::new(&mut app.maximum_tempo, app.minimum_tempo..=120).text("Max"));

    ui.label("Practice:");
    ui.separator();

    ui.add(egui::Slider::new(&mut app.practice_length, 0..=600).text("Practice Length"));
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
