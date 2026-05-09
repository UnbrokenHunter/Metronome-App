mod load;
mod save;
mod versioned;

use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;

use crate::app::{
    data::{
        accents, general, practice, settings, AppAccentPresetData, AppPracticeData,
        AppSaveData, AppSettingsData,
    },
    AppData,
};

use load::{load_default_config, load_user_or_default_config};
use save::save_versioned_json;

const LOCAL_CONFIG_DIR: &str = "config";

const PARAMETERS_FILE: &str = "mn_parameters.json";
const SETTINGS_FILE: &str = "mn_settings.json";
const PRACTICE_FILE: &str = "mn_practice.json";
const ACCENT_PRESETS_FILE: &str = "mn_accent_presets.json";

impl AppData {
    pub(crate) fn load_parameters() -> AppSaveData {
        load_user_or_default_config(
            &config_file(PARAMETERS_FILE),
            general::DEFAULT_JSON,
            general::VERSION,
            general::migrations::migrate,
        )
    }

    pub(crate) fn load_settings() -> AppSettingsData {
        load_user_or_default_config(
            &config_file(SETTINGS_FILE),
            settings::DEFAULT_JSON,
            settings::VERSION,
            settings::migrations::migrate,
        )
    }

    pub(crate) fn load_practice() -> AppPracticeData {
        load_user_or_default_config(
            &config_file(PRACTICE_FILE),
            practice::DEFAULT_JSON,
            practice::VERSION,
            practice::migrations::migrate,
        )
    }

    pub(crate) fn load_accent_presets() -> AppAccentPresetData {
        load_user_or_default_config(
            &config_file(ACCENT_PRESETS_FILE),
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
        if ensure_config_dir().is_none() {
            eprintln!("Failed to create MetronomeApp config directory.");
            return;
        }

        save_versioned_json(
            &self.parameters,
            &config_file(PARAMETERS_FILE),
            general::VERSION,
        );

        save_versioned_json(
            &self.settings,
            &config_file(SETTINGS_FILE),
            settings::VERSION,
        );

        save_versioned_json(
            &self.practice,
            &config_file(PRACTICE_FILE),
            practice::VERSION,
        );

        save_versioned_json(
            &self.accent_presets,
            &config_file(ACCENT_PRESETS_FILE),
            accents::VERSION,
        );
    }
}

fn config_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        return PathBuf::from("config");
    }

    ProjectDirs::from("com", "Bazooka", "MetronomeApp")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("config"))
}

fn ensure_config_dir() -> Option<PathBuf> {
    let dir = config_dir();

    fs::create_dir_all(&dir).ok()?;

    Some(dir)
}

fn config_file(file_name: &str) -> PathBuf {
    config_dir().join(file_name)
}
