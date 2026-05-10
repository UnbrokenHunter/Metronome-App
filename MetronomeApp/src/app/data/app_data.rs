use crate::app::data::runtime::{default_runtime_data, default_time_data};
use crate::app::data::themes::Theme;
use crate::app::data::{
    AppAccentPresetData, AppPracticeData, AppRunningData, AppSaveData, AppSettingsData,
    AppThemeData,
};
use crate::app::features::try_add_log;

pub struct AppData {
    pub parameters: AppSaveData,
    pub runtime: AppRunningData,
    pub settings: AppSettingsData,
    pub practice: AppPracticeData,
    pub accent_presets: AppAccentPresetData,
    pub themes: AppThemeData,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            parameters: Self::load_parameters(),
            runtime: default_runtime_data(), // Not saved on self because the others are loaded from disk in `persistence.rs`
            settings: Self::load_settings(),
            practice: Self::load_practice(),
            accent_presets: Self::load_accent_presets(),
            themes: Self::load_themes(),
        }
    }
}

impl Drop for AppData {
    fn drop(&mut self) {
        try_add_log(self, None);

        if let Err(error) = self.save() {
            eprintln!("{error}");
        }
    }
}

impl AppData {
    pub fn reset_metronome(&mut self) {
        self.runtime.playing = false;
        self.runtime.tempo = 100.0;
        self.runtime.points.clear();

        self.runtime.time_data = default_time_data();

        self.parameters.tempo_params.manual_offset = 0.0;
        self.parameters.tempo_params.manual_time_offset = 0.0;

        self.runtime.last_click_time = 0;
        self.runtime.last_subdivision_time = 0;
        self.runtime.last_click_accent = 0;
        self.runtime.last_tap_tempo_click = 0;

        self.runtime.pending_delete_log = None;
    }

    pub fn reset_all_parameters(&mut self) {
        self.runtime = default_runtime_data();
        self.parameters = Self::load_default_parameters();
    }

    pub fn current_theme(&self) -> &Theme {
        self.themes.get(self.settings.selected_theme_index)
    }
}
