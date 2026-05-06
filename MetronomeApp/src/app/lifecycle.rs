use crate::app::{
    logic::{clock, logs},
    types::{AppData, AppRunningData, TimeData},
};

impl Default for AppData {
    fn default() -> Self {
        Self {
            parameters: Self::load_parameters(),
            runtime: Self::default_runtime_data(),
            settings: Self::load_settings(),
            practice: Self::load_practice(),
            accent_presets: Self::load_accent_presets(),
        }
    }
}

impl AppData {
    pub fn reset_metronome(&mut self) {
        self.runtime.playing = false;
        self.runtime.tempo = 100.0;
        self.runtime.points.clear();

        self.runtime.time_data = Self::default_time_data();

        self.parameters.tempo_params.manual_offset = 0.0;
        self.parameters.tempo_params.manual_time_offset = 0.0;

        self.runtime.last_click_time = 0;
        self.runtime.last_subdivision_time = 0;
        self.runtime.last_click_accent = 0;
        self.runtime.last_tap_tempo_click = 0;
        self.runtime.menu_state = 0;
    }

    pub fn reset_all_parameters(&mut self) {
        self.runtime = Self::default_runtime_data();
        self.parameters = Self::load_default_parameters();
    }

    pub fn reset_settings(&mut self) {
        self.settings = Self::load_default_settings();
    }

    pub fn reset_accent_presets(&mut self) {
        self.accent_presets = Self::load_default_accent_presets();
    }

    #[allow(dead_code)]
    pub fn reset_practice_data(&mut self) {
        self.practice = Self::load_default_practice();
    }

    pub(crate) fn default_runtime_data() -> AppRunningData {
        AppRunningData {
            playing: false,
            audio: None,
            points: Vec::new(),
            last_click_time: 0,
            last_subdivision_time: 0,
            last_click_accent: 0,
            tempo: 120.0,
            last_tap_tempo_click: 0,
            time_data: Self::default_time_data(),
            menu_state: 0,
        }
    }

    pub(crate) fn default_time_data() -> TimeData {
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
}

impl Drop for AppData {
    fn drop(&mut self) {
        logs::try_add_log(self);
        self.save();
    }
}
