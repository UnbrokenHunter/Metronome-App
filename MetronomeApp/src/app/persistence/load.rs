use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;

use super::versioned::VersionedConfig;

pub(super) fn load_user_or_default_config<T>(
    user_path: &str,
    default_json: &str,
    current_version: u32,
    migrate: fn(&str, u32) -> Option<T>,
) -> T
where
    T: DeserializeOwned,
{
    if let Some(user_config) = load_config_file(user_path, current_version, migrate) {
        return user_config;
    }

    load_default_config(default_json, current_version, migrate)
}

pub(super) fn load_default_config<T>(
    default_json: &str,
    current_version: u32,
    migrate: fn(&str, u32) -> Option<T>,
) -> T
where
    T: DeserializeOwned,
{
    load_config_from_str(default_json, current_version, migrate)
        .expect("bundled default config JSON is invalid")
}

fn load_config_file<T>(
    path: &str,
    current_version: u32,
    migrate: fn(&str, u32) -> Option<T>,
) -> Option<T>
where
    T: DeserializeOwned,
{
    if !Path::new(path).exists() {
        return None;
    }

    let contents = fs::read_to_string(path).ok()?;
    load_config_from_str(&contents, current_version, migrate)
}

fn load_config_from_str<T>(
    contents: &str,
    current_version: u32,
    migrate: fn(&str, u32) -> Option<T>,
) -> Option<T>
where
    T: DeserializeOwned,
{
    if let Ok(versioned) = serde_json::from_str::<VersionedConfig<T>>(contents) {
        if versioned.version == current_version {
            return Some(versioned.data);
        }

        return migrate(contents, current_version);
    }

    // Legacy unversioned format.
    serde_json::from_str::<T>(contents).ok()
}
