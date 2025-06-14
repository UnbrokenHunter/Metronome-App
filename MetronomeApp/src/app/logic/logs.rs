use crate::app::{AppData, logic::clock, types::PracticeLog};

pub fn try_add_log(app: &mut AppData) {
    let duration_ms = app.runtime.time_data.calculated_time_since_start as u64;
    let min_tempo = app.parameters.tempo_params.min;
    let max_tempo = app.parameters.tempo_params.min;
    let datapoints = app.runtime.points.clone();
    if duration_ms > app.settings.min_practice_length && app.settings.save_logs {
        let now = clock::current_time();

        // Calculate Average Tempo
        let tempos: Vec<f64> = app.runtime.points.iter().map(|pair| pair[1]).collect();
        let average_tempo = (tempos.iter().sum::<f64>() / tempos.len() as f64) as f32;

        // Caluclate Average Delta
        let mut deltas_sum: f64 = 0.0;
        for i in 1..app.runtime.points.len() {
            let delta = (app.runtime.points[i][1] - app.runtime.points[i - 1][1])
                / (app.runtime.points[i][0] - app.runtime.points[i - 1][0]);
            deltas_sum += delta;
        }
        let average_delta = (deltas_sum / (app.runtime.points.len() - 1) as f64) as f32;

        let points: Vec<[f64; 2]> = if app.settings.save_plots {
            let step = match app.settings.plot_granularity {
                0 => 4,
                1 => 3,
                2 => 2,
                3 => 1,
                _ => 1,
            };

            let mut condensed: Vec<[f64; 2]> = datapoints.iter().step_by(step).cloned().collect();

            if let Some(last) = datapoints.last() {
                if condensed.last() != Some(last) {
                    condensed.push(*last);
                }
            }

            condensed
        } else {
            Vec::new()
        };

        let title = "".to_owned();
        let notes = "".to_owned();

        app.practice.logs.push(PracticeLog {
            time_started: now,
            duration_ms,
            min_tempo,
            max_tempo,
            average_tempo,
            average_delta,
            points,
            title,
            notes,
        });
        println!("Add Log");
    }
}
