use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Serialize, de::DeserializeOwned};

use crate::app::types::{
    AppAccentPresetData, AppData, AppPracticeData, AppSaveData, AppSettingsData,
};

const LOCAL_CONFIG_DIR: &str = "config";

const PARAMETERS_FILE: &str = "mn_parameters.json";
const SETTINGS_FILE: &str = "mn_settings.json";
const PRACTICE_FILE: &str = "mn_practice.json";
const ACCENT_PRESETS_FILE: &str = "mn_accent_presets.json";

impl AppData {
    pub(crate) fn load_parameters() -> AppSaveData {
        load_json(&config_file(PARAMETERS_FILE)).unwrap_or_else(Self::default_parameters_data)
    }

    pub(super) fn load_settings() -> AppSettingsData {
        load_json(&config_file(SETTINGS_FILE)).unwrap_or_else(Self::default_settings_data)
    }

    pub(super) fn load_practice() -> AppPracticeData {
        load_json(&config_file(PRACTICE_FILE)).unwrap_or_else(Self::default_practice_data)
    }

    pub(super) fn load_accent_presets() -> AppAccentPresetData {
        load_json(&config_file(ACCENT_PRESETS_FILE))
            .unwrap_or_else(Self::default_accent_presets_data)
    }

    pub(crate) fn save(&self) {
        if ensure_config_dir().is_none() {
            eprintln!("Failed to create MetronomeApp config directory.");
            return;
        }

        save_json(&self.parameters, &config_file(PARAMETERS_FILE));
        save_json(&self.settings, &config_file(SETTINGS_FILE));
        save_json(&self.accent_presets, &config_file(ACCENT_PRESETS_FILE));
        save_json(&self.practice, &config_file(PRACTICE_FILE));
    }
}

fn config_dir() -> PathBuf {
    let local_config = Path::new(LOCAL_CONFIG_DIR);

    if local_config.exists() {
        return local_config.to_path_buf();
    }

    ProjectDirs::from("com", "Bazooka", "Metronome")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| local_config.to_path_buf())
}

fn ensure_config_dir() -> Option<PathBuf> {
    let dir = config_dir();

    fs::create_dir_all(&dir).ok()?;

    Some(dir)
}

fn config_file(file_name: &str) -> PathBuf {
    config_dir().join(file_name)
}

fn load_json<T: DeserializeOwned>(path: &Path) -> Option<T> {
    if !path.exists() {
        return None;
    }

    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn save_json<T: Serialize>(value: &T, path: &Path) {
    if let Ok(json) = serde_json::to_string_pretty(value)
        && let Ok(mut file) = fs::File::create(path)
    {
        let _ = file.write_all(json.as_bytes());
    }
}
