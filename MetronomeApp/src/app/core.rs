use std::time::{SystemTime, UNIX_EPOCH};

use eframe::Frame;
use eframe::egui::Context;

use crate::app::types::AppSettingsData;

use super::logic::metronome;
use super::logic::{clock, keyboard};
use super::types::{AppData, AppPracticeData, AppRunningData, AppSaveData, PracticeLog, TimeData};
use super::ui::layout;
use super::{GrowthType, Menus, Sounds, TempoParams};

use serde_json;
use std::fs;
use std::io::Write;
use std::path::Path;

impl Default for AppData {
    fn default() -> Self {
        let parameters_path: &'static str = "parameters.json";
        let settings_path: &'static str = "settings.json";
        let practice_path: &'static str = "practice.json";

        let save: AppSaveData = if Path::new(parameters_path).exists() {
            if let Ok(contents) = fs::read_to_string(parameters_path) {
                serde_json::from_str::<AppSaveData>(&contents)
                    .unwrap_or_else(|_| AppData::default_parameters_data())
            } else {
                AppData::default_parameters_data()
            }
        } else {
            AppData::default_parameters_data()
        };

        let settings: AppSettingsData = if Path::new(settings_path).exists() {
            if let Ok(contents) = fs::read_to_string(settings_path) {
                serde_json::from_str::<AppSettingsData>(&contents)
                    .unwrap_or_else(|_| AppData::default_settings_data())
            } else {
                AppData::default_settings_data()
            }
        } else {
            AppData::default_settings_data()
        };

        let practice = if Path::new(practice_path).exists() {
            if let Ok(contents) = fs::read_to_string(practice_path) {
                serde_json::from_str::<AppPracticeData>(&contents)
                    .unwrap_or_else(|_| AppPracticeData { logs: Vec::new() })
            } else {
                AppPracticeData { logs: Vec::new() }
            }
        } else {
            AppPracticeData { logs: Vec::new() }
        };

        AppData {
            parameters: save,
            runtime: AppData::default_runtime_data(),
            settings,
            practice,
        }
    }
}

impl AppData {
    pub fn reset_metronome(&mut self) {
        self.runtime.playing = false;
        self.runtime.tempo = 100.0;
        self.runtime.points.clear();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.runtime.time_data = crate::app::types::TimeData {
            time: now,
            start_time: now,
            time_since_start: 0,
            delta_time: 0,
            paused_time: 0,
            calculated_time_since_start: 0,
        };

        self.parameters.tempo_params.manual_offset = 0.0;
        self.parameters.tempo_params.manual_time_offset = 0.0;
        self.runtime.last_click_time = 0;
    }

    pub fn reset_all_parameters(&mut self) {
        self.runtime = Self::default_runtime_data();
        self.parameters = Self::default_parameters_data();
    }

    pub fn reset_settings(&mut self) {
        self.settings = Self::default_settings_data();
    }

    fn default_parameters_data() -> AppSaveData {
        AppSaveData {
            tempo_params: TempoParams {
                min: 100,
                max: 150,
                length: 5.0,
                scaler: 0.5,
                manual_offset: 0.0,
                manual_time_offset: 0.0,
            },
            sound: Sounds::Beep,
            volume: 0.7,
            growth_type: GrowthType::Linear,
            infinte: false,
        }
    }

    fn default_runtime_data() -> AppRunningData {
        let now = Self::current_time();
        AppRunningData {
            playing: false,
            audio: None,
            points: Vec::new(),
            last_click_time: 0,
            tempo: 120.0,
            last_tap_tempo_click: 0,
            menu: Menus::Metronome,
            time_data: TimeData {
                time: now,
                start_time: now,
                time_since_start: 0,
                delta_time: 0,
                paused_time: 0,
                calculated_time_since_start: 0,
            },
            menu_state: 0,
        }
    }

    fn default_settings_data() -> AppSettingsData {
        AppSettingsData {
            save_logs: (true),
            save_plots: (true),
            plot_granularity: (2),
        }
    }

    fn current_time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    pub fn try_add_log(
        &mut self,
        duration_ms: u64,
        min_tempo: u32,
        max_tempo: u32,
        datapoints: Vec<[f64; 2]>,
        settings: AppSettingsData,
    ) {
        if duration_ms > 0 && settings.save_logs {
            let now = Self::current_time();

            // Calculate Average Tempo
            let tempos: Vec<f64> = self.runtime.points.iter().map(|pair| pair[1]).collect();
            let average_tempo = (tempos.iter().sum::<f64>() / tempos.len() as f64) as f32;

            // Caluclate Average Delta
            let mut deltas_sum: f64 = 0.0;
            for i in 1..self.runtime.points.len() {
                let delta = (self.runtime.points[i][1] - self.runtime.points[i - 1][1])
                    / (self.runtime.points[i][0] - self.runtime.points[i - 1][0]);
                deltas_sum += delta;
            }
            let average_delta = (deltas_sum / (self.runtime.points.len() - 1) as f64) as f32;

            let points: Vec<[f64; 2]> = if settings.save_plots {
                let step = match settings.plot_granularity {
                    0 => 4,
                    1 => 3,
                    2 => 2,
                    3 => 1,
                    _ => 1,
                };

                let mut condensed: Vec<[f64; 2]> =
                    datapoints.iter().step_by(step).cloned().collect();

                if let Some(last) = datapoints.last() {
                    if condensed.last() != Some(last) {
                        condensed.push(*last);
                    }
                }

                condensed
            } else {
                Vec::new()
            };

            self.practice.logs.push(PracticeLog {
                time_started: now,
                duration_ms,
                min_tempo,
                max_tempo,
                average_tempo,
                average_delta,
                points,
            });
            println!("Add Log");
        }
    }
}

impl Drop for AppData {
    fn drop(&mut self) {
        // Save the log if app is closed without reseting
        self.try_add_log(
            self.runtime.time_data.calculated_time_since_start as u64,
            self.parameters.tempo_params.min,
            self.parameters.tempo_params.max,
            self.runtime.points.clone(),
            self.settings,
        );

        // Serialize only the `save` part
        if let Ok(json) = serde_json::to_string_pretty(&self.parameters) {
            if let Ok(mut file) = fs::File::create("parameters.json") {
                let _ = file.write_all(json.as_bytes());
            }
        }

        if let Ok(json) = serde_json::to_string_pretty(&self.settings) {
            if let Ok(mut file) = fs::File::create("settings.json") {
                let _ = file.write_all(json.as_bytes());
            }
        }

        if let Ok(json) = serde_json::to_string_pretty(&self.practice) {
            if let Ok(mut file) = fs::File::create("practice.json") {
                let _ = file.write_all(json.as_bytes());
            }
        }
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        layout::layout(self, ctx);

        keyboard::check_keyboard(self, ctx.clone());
        clock::update_time(&mut self.runtime.time_data, self.runtime.playing);
        metronome::update_metronome(self);
        ctx.request_repaint();
    }
}
