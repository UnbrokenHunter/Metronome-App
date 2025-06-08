use crate::app::AppData;
use crate::app::logic::functions::calculate;

pub fn calculate_tempo(app: &mut AppData) {
    if app.runtime.playing {
        let x = app.runtime.time_data.calculated_time_since_start as f64 / 1000.0
            + app.parameters.tempo_params.manual_time_offset;
        app.runtime.tempo = calculate(app.parameters.growth_type, x, app.parameters.tempo_params);

        // Clamp Values
        if !app.parameters.infinte {
            if app.runtime.tempo > app.parameters.tempo_params.max as f64 {
                app.runtime.tempo = app.parameters.tempo_params.max as f64;
            }
            if app.runtime.tempo < app.parameters.tempo_params.min as f64 {
                app.runtime.tempo = app.parameters.tempo_params.min as f64;
            }
        }

        app.runtime.tempo += app.parameters.tempo_params.manual_offset;

        app.runtime.points.push([
            app.runtime.time_data.calculated_time_since_start as f64 / 1000.0,
            app.runtime.tempo,
        ]);
    }
}
