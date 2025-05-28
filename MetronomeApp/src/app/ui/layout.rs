use crate::app::MyApp;
use crate::app::logic::functions::calculate;
use crate::app::ui::plot::draw_plot;
use crate::app::ui::settings;
use eframe::egui::{self, Ui};

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    settings::practice_ui(app, ui);
    settings::tempo_ui(app, ui);

    settings::growth_ui(app, ui);
    settings::sound_ui(app, ui);
}

pub fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Tempo");

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

        println!("{}", app.time_data.calculated_time_since_start.to_string());

        draw_plot(ui, &app.points, app.tempo_params);

        if ui.add(egui::Button::new("Play")).clicked() {
            app.playing = !app.playing
        }
    });
}
