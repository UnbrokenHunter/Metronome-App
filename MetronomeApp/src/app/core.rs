use std::time::{SystemTime, UNIX_EPOCH};

use eframe::Frame;
use eframe::egui::Context;

use crate::app::logic::logs;
use crate::app::types::{
    AccentChain, AccentData, AppAccentPresetData, AppSettingsData, BeatData, BeatState, ColorScheme,
};

use super::logic::metronome;
use super::logic::{clock, keyboard};
use super::types::{AppData, AppPracticeData, AppRunningData, AppSaveData, TimeData};
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
        let accent_presets_path: &'static str = "accent_presets.json";

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

        let accent_presets: AppAccentPresetData = if Path::new(accent_presets_path).exists() {
            if let Ok(contents) = fs::read_to_string(accent_presets_path) {
                serde_json::from_str::<AppAccentPresetData>(&contents)
                    .unwrap_or_else(|_| AppData::default_accent_presets_data())
            } else {
                AppData::default_accent_presets_data()
            }
        } else {
            AppData::default_accent_presets_data()
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
            accent_presets,
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
        self.runtime.last_subdivision_time = 0;
        self.runtime.last_click_accent = 0;
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
            accents: AccentChain {
                name: "Untitled Accent Chain".to_owned(),
                accents: vec![
                    AccentData {
                        beats: vec![
                            BeatData {
                                state: BeatState::Downbeat,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                            BeatData {
                                state: BeatState::Strong,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                        ],
                        subdivision: 2,
                        name: "".to_owned(),
                    },
                    AccentData {
                        beats: vec![
                            BeatData {
                                state: BeatState::Strong,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                            BeatData {
                                state: BeatState::Strong,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                        ],
                        subdivision: 2,
                        name: "".to_owned(),
                    },
                ],
            },
        }
    }

    fn default_runtime_data() -> AppRunningData {
        let now = clock::current_time();
        AppRunningData {
            playing: false,
            audio: None,
            points: Vec::new(),
            last_click_time: 0,
            tempo: 120.0,
            last_tap_tempo_click: 0,
            last_subdivision_time: 0,
            last_click_accent: 0,
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
            min_practice_length: (5000),
            color_scheme: ColorScheme {
                name: "Dark".to_owned(),
                override_color: "#3C4664".to_owned(), // (60, 70, 100)
                downbeat_color: "#505078".to_owned(), // (80, 80, 120)
                strong_color: "#828282".to_owned(),   // (130, 130, 130)
                weak_color: "#505050".to_owned(),     // (80, 80, 80)
                off_color: "#282828".to_owned(),      // (40, 40, 40)
            },
        }
    }

    fn default_accent_presets_data() -> AppAccentPresetData {
        AppAccentPresetData {
            accent_chains: (vec![
                AccentChain {
                    name: "4/4".to_owned(),
                    accents: vec![AccentData {
                        beats: vec![
                            BeatData {
                                state: BeatState::Downbeat,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                            BeatData {
                                state: BeatState::Strong,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                        ],
                        subdivision: 2,
                        name: "".to_owned(),
                    }],
                },
                AccentChain {
                    name: "3/4".to_owned(),
                    accents: vec![AccentData {
                        beats: vec![
                            BeatData {
                                state: BeatState::Downbeat,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                        ],
                        subdivision: 2,
                        name: "".to_owned(),
                    }],
                },
                AccentChain {
                    name: "6/8".to_owned(),
                    accents: vec![AccentData {
                        beats: vec![
                            BeatData {
                                state: BeatState::Downbeat,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                        ],
                        subdivision: 3,
                        name: "".to_owned(),
                    }],
                },
                AccentChain {
                    name: "5/4".to_owned(),
                    accents: vec![AccentData {
                        beats: vec![
                            BeatData {
                                state: BeatState::Downbeat,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                            BeatData {
                                state: BeatState::Weak,
                            },
                        ],
                        subdivision: 1,
                        name: "".to_owned(),
                    }],
                },
            ]),
        }
    }
}

impl Drop for AppData {
    fn drop(&mut self) {
        // Save the log if app is closed without reseting
        logs::try_add_log(self);

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

        if let Ok(json) = serde_json::to_string_pretty(&self.accent_presets) {
            if let Ok(mut file) = fs::File::create("accent_presets.json") {
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
        self.settings.color_scheme.apply_to_ctx(ctx); // This probably dosnt need to happen evey fram but idk
        layout::layout(self, ctx);

        keyboard::check_keyboard(self, ctx.clone());
        clock::update_time(&mut self.runtime.time_data, self.runtime.playing);
        metronome::update_metronome(self);
        ctx.request_repaint();
    }
}
