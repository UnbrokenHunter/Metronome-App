use std::time::{SystemTime, UNIX_EPOCH};

use eframe::Frame;
use eframe::egui::Context;

use super::logic::metronome;
use super::logic::{clock, keyboard};
use super::types::{AppData, AppRunningData, AppSaveData, TimeData};
use super::ui::layout;
use super::{GrowthType, Sounds, TempoParams};

use serde_json;
use std::fs;
use std::io::Write;
use std::path::Path;

impl Default for AppData {
    fn default() -> Self {
        let save_path = "settings.json";

        // Try to load from save file
        if Path::new(save_path).exists() {
            if let Ok(contents) = fs::read_to_string(save_path) {
                if let Ok(save_data) = serde_json::from_str::<AppSaveData>(&contents) {
                    return AppData {
                        save: save_data,
                        runtime: AppRunningData {
                            points: Vec::new(),
                            audio: None,
                            playing: false,
                            last_click_time: 0,
                        },
                    };
                }
            }
        }

        // Fall back to defaults
        AppData {
            save: Self::default_save_data(),
            runtime: Self::default_runtime_data(),
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
}

impl Drop for AppData {
    fn drop(&mut self) {
        // Serialize only the `save` part
        if let Ok(json) = serde_json::to_string_pretty(&self.save) {
            if let Ok(mut file) = fs::File::create("settings.json") {
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
