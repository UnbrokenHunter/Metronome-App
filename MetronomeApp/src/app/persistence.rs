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

const DEFAULT_PARAMETERS_JSON: &str = include_str!("../../assets/defaults/mn_parameters.json");

const DEFAULT_SETTINGS_JSON: &str = include_str!("../../assets/defaults/mn_settings.json");

const DEFAULT_PRACTICE_JSON: &str = include_str!("../../assets/defaults/mn_practice.json");

const DEFAULT_ACCENT_PRESETS_JSON: &str =
    include_str!("../../assets/defaults/mn_accent_presets.json");

impl AppData {
    pub(crate) fn load_parameters() -> AppSaveData {
        load_or_default_json(PARAMETERS_PATH, DEFAULT_PARAMETERS_JSON)
            .expect("bundled default parameters JSON is invalid")
    }

    pub(crate) fn load_settings() -> AppSettingsData {
        load_or_default_json(SETTINGS_PATH, DEFAULT_SETTINGS_JSON)
            .expect("bundled default settings JSON is invalid")
    }

    pub(crate) fn load_practice() -> AppPracticeData {
        load_or_default_json(PRACTICE_PATH, DEFAULT_PRACTICE_JSON)
            .expect("bundled default practice JSON is invalid")
    }

    pub(crate) fn load_accent_presets() -> AppAccentPresetData {
        load_or_default_json(ACCENT_PRESETS_PATH, DEFAULT_ACCENT_PRESETS_JSON)
            .expect("bundled default accent presets JSON is invalid")
    }

    pub(crate) fn load_default_parameters() -> AppSaveData {
        load_default_json(DEFAULT_PARAMETERS_JSON)
            .expect("bundled default parameters JSON is invalid")
    }

    pub(crate) fn load_default_settings() -> AppSettingsData {
        load_default_json(DEFAULT_SETTINGS_JSON).expect("bundled default settings JSON is invalid")
    }

    #[allow(dead_code)]
    pub(crate) fn load_default_practice() -> AppPracticeData {
        load_default_json(DEFAULT_PRACTICE_JSON).expect("bundled default practice JSON is invalid")
    }

    pub(crate) fn load_default_accent_presets() -> AppAccentPresetData {
        load_default_json(DEFAULT_ACCENT_PRESETS_JSON)
            .expect("bundled default accent presets JSON is invalid")
    }

    pub(crate) fn save(&self) {
        let _ = fs::create_dir_all(CONFIG_DIR);

        save_json(&self.parameters, PARAMETERS_PATH);
        save_json(&self.settings, SETTINGS_PATH);
        save_json(&self.practice, PRACTICE_PATH);
        save_json(&self.accent_presets, ACCENT_PRESETS_PATH);
    }
}

fn load_or_default_json<T>(user_path: &str, default_json: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    load_json_file(user_path).or_else(|| load_default_json(default_json))
}

fn load_json_file<T>(path: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    if !Path::new(path).exists() {
        return None;
    }

    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_default_json<T>(default_json: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(default_json).ok()
}

fn save_json<T>(value: &T, path: &str)
where
    T: Serialize,
{
    if let Ok(json) = serde_json::to_string_pretty(value)
        && let Ok(mut file) = fs::File::create(path)
    {
        let _ = file.write_all(json.as_bytes());
    }
}
