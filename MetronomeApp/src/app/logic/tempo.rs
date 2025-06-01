use crate::app::MyApp;
use crate::app::logic::functions::calculate;

pub fn calculate_tempo(app: &mut MyApp) {
    if app.playing {
        let x = app.time_data.calculated_time_since_start as f64 / 1000.0
            + app.tempo_params.manual_time_offset;
        app.tempo = calculate(app.growth_type, x, app.tempo_params);

        // Clamp Values
        if !app.infinte {
            if app.tempo > app.tempo_params.max as f64 {
                app.tempo = app.tempo_params.max as f64;
            }
            if app.tempo < app.tempo_params.min as f64 {
                app.tempo = app.tempo_params.min as f64;
            }
        }

        app.tempo += app.tempo_params.manual_offset;

        app.points.push([
            app.time_data.calculated_time_since_start as f64 / 1000.0,
            app.tempo,
        ]);
    }
}
