use crate::app::AppData;
use crate::app::logic::functions::calculate;

pub fn calculate_tempo(app: &mut AppData) {
    if app.runtime.playing {
        let x = app.save.time_data.calculated_time_since_start as f64 / 1000.0
            + app.save.tempo_params.manual_time_offset;
        app.save.tempo = calculate(app.save.growth_type, x, app.save.tempo_params);

        // Clamp Values
        if !app.save.infinte {
            if app.save.tempo > app.save.tempo_params.max as f64 {
                app.save.tempo = app.save.tempo_params.max as f64;
            }
            if app.save.tempo < app.save.tempo_params.min as f64 {
                app.save.tempo = app.save.tempo_params.min as f64;
            }
        }

        app.save.tempo += app.save.tempo_params.manual_offset;

        app.runtime.points.push([
            app.save.time_data.calculated_time_since_start as f64 / 1000.0,
            app.save.tempo,
        ]);
    }
}
