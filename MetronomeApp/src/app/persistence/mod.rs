mod load;
mod save;
mod versioned;

use crate::app::{
    data::{
        accents, general, practice, settings, AppAccentPresetData, AppPracticeData,
        AppSaveData, AppSettingsData,
    },
    AppData,
};

use load::{load_default_config, load_user_or_default_config};
use save::save_versioned_json;

const CONFIG_DIR: &str = "config";

const PARAMETERS_PATH: &str = "config/mn_parameters.json";
const SETTINGS_PATH: &str = "config/mn_settings.json";
const PRACTICE_PATH: &str = "config/mn_practice.json";
const ACCENT_PRESETS_PATH: &str = "config/mn_accent_presets.json";

impl AppData {
    pub(crate) fn load_parameters() -> AppSaveData {
        load_user_or_default_config(
            PARAMETERS_PATH,
            general::DEFAULT_JSON,
            general::VERSION,
            general::migrations::migrate,
        )
    }

    pub(crate) fn load_settings() -> AppSettingsData {
        load_user_or_default_config(
            SETTINGS_PATH,
            settings::DEFAULT_JSON,
            settings::VERSION,
            settings::migrations::migrate,
        )
    }

    pub(crate) fn load_practice() -> AppPracticeData {
        load_user_or_default_config(
            PRACTICE_PATH,
            practice::DEFAULT_JSON,
            practice::VERSION,
            practice::migrations::migrate,
        )
    }

    pub(crate) fn load_accent_presets() -> AppAccentPresetData {
        load_user_or_default_config(
            ACCENT_PRESETS_PATH,
            accents::DEFAULT_JSON,
            accents::VERSION,
            accents::migrations::migrate,
        )
    }

    pub(crate) fn load_default_parameters() -> AppSaveData {
        load_default_config(
            general::DEFAULT_JSON,
            general::VERSION,
            general::migrations::migrate,
        )
    }

    pub(crate) fn load_default_settings() -> AppSettingsData {
        load_default_config(
            settings::DEFAULT_JSON,
            settings::VERSION,
            settings::migrations::migrate,
        )
    }

    #[allow(dead_code)]
    pub(crate) fn load_default_practice() -> AppPracticeData {
        load_default_config(
            practice::DEFAULT_JSON,
            practice::VERSION,
            practice::migrations::migrate,
        )
    }

    pub(crate) fn load_default_accent_presets() -> AppAccentPresetData {
        load_default_config(
            accents::DEFAULT_JSON,
            accents::VERSION,
            accents::migrations::migrate,
        )
    }

    pub(crate) fn save(&self) {
        let _ = std::fs::create_dir_all(CONFIG_DIR);

        save_versioned_json(&self.parameters, PARAMETERS_PATH, general::VERSION);
        save_versioned_json(&self.settings, SETTINGS_PATH, settings::VERSION);
        save_versioned_json(&self.practice, PRACTICE_PATH, practice::VERSION);
        save_versioned_json(&self.accent_presets, ACCENT_PRESETS_PATH, accents::VERSION);
    }
}
