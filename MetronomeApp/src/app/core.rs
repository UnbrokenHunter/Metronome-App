use std::time::{SystemTime, UNIX_EPOCH};

use eframe::Frame;
use eframe::egui::Context;

use super::logic::metronome;
use super::logic::{clock, keyboard};
use super::types::{AppData, AppPracticeData, AppRunningData, AppSaveData, PracticeLog, TimeData};
use super::ui::layout;
use super::{GrowthType, Sounds, TempoParams};

use serde_json;
use std::fs;
use std::io::Write;
use std::path::Path;

impl Default for AppData {
    fn default() -> Self {
        let settings_path: &'static str = "settings.json";
        let practice_path: &'static str = "practice.json";

        let save = if Path::new(settings_path).exists() {
            if let Ok(contents) = fs::read_to_string(settings_path) {
                serde_json::from_str::<AppSaveData>(&contents)
                    .unwrap_or_else(|_| AppData::default_save_data())
            } else {
                AppData::default_save_data()
            }
        } else {
            AppData::default_save_data()
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
            save,
            runtime: AppData::default_runtime_data(),
            practice,
        }
    }
}

impl AppData {
    pub fn reset_metronome(&mut self) {
        self.runtime.playing = false;
        self.save.tempo = 100.0;
        self.runtime.points.clear();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.save.time_data = crate::app::types::TimeData {
            time: now,
            start_time: now,
            time_since_start: 0,
            delta_time: 0,
            paused_time: 0,
            calculated_time_since_start: 0,
        };

        self.save.tempo_params.manual_offset = 0.0;
        self.save.tempo_params.manual_time_offset = 0.0;
        self.runtime.last_click_time = 0;
    }

    pub fn reset_all_settings(&mut self) {
        self.runtime = Self::default_runtime_data();
        self.save = Self::default_save_data();
    }

    fn default_save_data() -> AppSaveData {
        let now = Self::current_time();
        AppSaveData {
            tempo: 100.0,
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
            time_data: TimeData {
                time: now,
                start_time: now,
                time_since_start: 0,
                delta_time: 0,
                paused_time: 0,
                calculated_time_since_start: 0,
            },
        }
    }

    fn default_runtime_data() -> AppRunningData {
        AppRunningData {
            playing: false,
            audio: None,
            points: Vec::new(),
            last_click_time: 0,
        }
    }

    fn current_time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    pub fn try_add_log(&mut self, duration_ms: u64, min_tempo: u32, max_tempo: u32) {
        if duration_ms > 0 {
            let now = Self::current_time();

            self.practice.logs.push(PracticeLog {
                time_started: now,
                duration_ms,
                min_tempo,
                max_tempo,
            });
            println!("Add Log")
        }
    }
}

impl Drop for AppData {
    fn drop(&mut self) {
        // Save the log if app is closed without reseting
        self.try_add_log(
            self.save.time_data.calculated_time_since_start as u64,
            self.save.tempo_params.min,
            self.save.tempo_params.max,
        );

        // Serialize only the `save` part
        if let Ok(json) = serde_json::to_string_pretty(&self.save) {
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
        clock::update_time(&mut self.save.time_data, self.runtime.playing);
        metronome::update_metronome(self);
        ctx.request_repaint();
    }
}
