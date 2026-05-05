use crate::app::{
    logic::clock, types::{
        AccentChain, AccentData, AppAccentPresetData, AppData, AppPracticeData, AppRunningData,
        AppSaveData, AppSettingsData, BeatData, BeatState, ColorScheme, TimeData,
    }, GrowthType, Menus,
    Sounds,
    TempoParams,
};

impl AppData {
    pub(crate) fn default_parameters_data() -> AppSaveData {
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
            infinite: false,
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
                        name: String::new(),
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
                        name: String::new(),
                    },
                ],
            },
        }
    }

    pub(super) fn default_runtime_data() -> AppRunningData {
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
            time_data: Self::default_time_data(),
            menu_state: 0,
        }
    }

    pub(super) fn default_time_data() -> TimeData {
        let now = clock::current_time();

        TimeData {
            time: now,
            start_time: now,
            time_since_start: 0,
            delta_time: 0,
            paused_time: 0,
            calculated_time_since_start: 0,
        }
    }

    pub(super) fn default_settings_data() -> AppSettingsData {
        AppSettingsData {
            save_logs: true,
            save_plots: true,
            plot_granularity: 2,
            min_practice_length: 5000,
            color_scheme: ColorScheme::dark(),
        }
    }

    pub(super) fn default_practice_data() -> AppPracticeData {
        AppPracticeData { logs: Vec::new() }
    }

    pub(super) fn default_accent_presets_data() -> AppAccentPresetData {
        AppAccentPresetData {
            accent_chains: vec![
                preset(
                    "4/4",
                    2,
                    &[
                        BeatState::Downbeat,
                        BeatState::Weak,
                        BeatState::Strong,
                        BeatState::Weak,
                    ],
                ),
                preset(
                    "3/4",
                    2,
                    &[BeatState::Downbeat, BeatState::Weak, BeatState::Weak],
                ),
                preset("6/8", 3, &[BeatState::Downbeat, BeatState::Weak]),
                preset(
                    "5/4",
                    1,
                    &[
                        BeatState::Downbeat,
                        BeatState::Weak,
                        BeatState::Weak,
                        BeatState::Weak,
                        BeatState::Weak,
                    ],
                ),
            ],
        }
    }
}

fn preset(name: &str, subdivision: u32, beats: &[BeatState]) -> AccentChain {
    AccentChain {
        name: name.to_owned(),
        accents: vec![AccentData {
            beats: beats
                .iter()
                .copied()
                .map(|state| BeatData { state })
                .collect(),
            subdivision,
            name: String::new(),
        }],
    }
}
