use std::fs;
use std::io::Write;
use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use crate::app::types::{
    AppAccentPresetData, AppData, AppPracticeData, AppSaveData, AppSettingsData,
};

const CONFIG_DIR: &str = "config";

const PARAMETERS_PATH: &str = "config/mn_parameters.json";
const SETTINGS_PATH: &str = "config/mn_settings.json";
const PRACTICE_PATH: &str = "config/mn_practice.json";
const ACCENT_PRESETS_PATH: &str = "config/mn_accent_presets.json";

impl AppData {
    pub(crate) fn load_parameters() -> AppSaveData {
        load_json(PARAMETERS_PATH).unwrap_or_else(Self::default_parameters_data)
    }

    pub(super) fn load_settings() -> AppSettingsData {
        load_json(SETTINGS_PATH).unwrap_or_else(Self::default_settings_data)
    }

    pub(super) fn load_practice() -> AppPracticeData {
        load_json(PRACTICE_PATH).unwrap_or_else(Self::default_practice_data)
    }

    pub(super) fn load_accent_presets() -> AppAccentPresetData {
        load_json(ACCENT_PRESETS_PATH).unwrap_or_else(Self::default_accent_presets_data)
    }

    pub(crate) fn save(&self) {
        let _ = fs::create_dir_all(CONFIG_DIR);

        save_json(&self.parameters, PARAMETERS_PATH);
        save_json(&self.settings, SETTINGS_PATH);
        save_json(&self.accent_presets, ACCENT_PRESETS_PATH);
        save_json(&self.practice, PRACTICE_PATH);
    }
}

fn load_json<T: DeserializeOwned>(path: &str) -> Option<T> {
    if !Path::new(path).exists() {
        return None;
    }

    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn save_json<T: Serialize>(value: &T, path: &str) {
    if let Ok(json) = serde_json::to_string_pretty(value)
        && let Ok(mut file) = fs::File::create(path)
    {
        let _ = file.write_all(json.as_bytes());
    }
}
