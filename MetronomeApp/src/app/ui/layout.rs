use crate::app::MyApp;
use crate::app::logic::functions::calculate;
use crate::app::ui::settings;
use eframe::egui::Ui;

pub fn settings_ui(app: &mut MyApp, ui: &mut Ui) {
    settings::practice_ui(app, ui);
    settings::tempo_ui(app, ui);

    settings::growth_ui(app, ui);
    settings::sound_ui(app, ui);
}

pub fn main_ui(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        calculate_tempo(app);

        settings::plot_ui(app, ui);

        ui.horizontal(|ui: &mut Ui| {
            settings::play_ui(app, ui);
            settings::info_ui(app, ui);
        });
    });
}

fn calculate_tempo(app: &mut MyApp) {
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

        app.tempo += app.tempo_params.manual_offset;

        app.points.push([
            app.time_data.calculated_time_since_start as f64 / 1000.0,
            app.tempo,
        ]);
    }
}
