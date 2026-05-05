use crate::app::logic::logs;
use crate::app::types::AppData;

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
    }

    pub fn reset_all_parameters(&mut self) {
        self.runtime = Self::default_runtime_data();
        self.parameters = Self::default_parameters_data();
    }

    pub fn reset_settings(&mut self) {
        self.settings = Self::default_settings_data();
    }

    pub fn reset_accent_presets(&mut self) {
        self.accent_presets = Self::default_accent_presets_data();
    }
}

impl Drop for AppData {
    fn drop(&mut self) {
        logs::try_add_log(self);
        self.save();
    }
}
