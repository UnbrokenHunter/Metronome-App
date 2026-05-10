use crate::app::{AppData, PracticeLog};

pub fn try_add_log(app: &mut AppData, title: Option<String>) {
    let duration_ms = app.runtime.time_data.calculated_time_since_start as u64;

    if duration_ms <= app.settings.min_practice_length || !app.settings.save_logs {
        println!("Log Save Canceled. Duration <= Length or Saving Disabled");
        return;
    }

    if app.runtime.points.len() < 2 {
        println!("Log Save Canceled. Points < 2");
        return;
    }

    let time_started = app.runtime.time_data.start_time;

    let min_tempo = app.parameters.tempo_params.min;
    let max_tempo = app.parameters.tempo_params.max;

    // Calculate average tempo
    let tempo_sum: f64 = app.runtime.points.iter().map(|point| point[1]).sum();
    let average_tempo = (tempo_sum / app.runtime.points.len() as f64) as f32;

    // Calculate average delta
    let mut deltas_sum: f64 = 0.0;
    let mut deltas_count: usize = 0;

    for i in 1..app.runtime.points.len() {
        let dx = app.runtime.points[i][0] - app.runtime.points[i - 1][0];
        let dy = app.runtime.points[i][1] - app.runtime.points[i - 1][1];

        if dx != 0.0 {
            deltas_sum += dy / dx;
            deltas_count += 1;
        }
    }

    let average_delta = if deltas_count > 0 {
        (deltas_sum / deltas_count as f64) as f32
    } else {
        0.0
    };

    let points: Vec<[f64; 2]> = if app.settings.save_plots {
        let step = match app.settings.plot_granularity {
            0 => 100,
            1 => 50,
            2 => 10,
            3 => 1,
            _ => 1,
        };

        let datapoints = &app.runtime.points;
        let mut condensed: Vec<[f64; 2]> = datapoints.iter().step_by(step).cloned().collect();

        if let Some(last) = datapoints.last()
            && condensed.last() != Some(last)
        {
            condensed.push(*last);
        }

        condensed
    } else {
        Vec::new()
    };

    app.practice.logs.push(PracticeLog {
        time_started,
        duration_ms,
        min_tempo,
        max_tempo,
        average_tempo,
        average_delta,
        points,
        title: title.unwrap_or_default(),
        notes: String::new(),
    });

    println!("Add Log");
}
