use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{fs, path::Path};

pub type MigrationFn<T> = fn(u32, Value) -> Option<T>;

pub(super) fn load_user_or_default_config<T>(
    path: &Path,
    default_json: &str,
    current_version: u32,
    migrate: MigrationFn<T>,
) -> T
where
    T: DeserializeOwned,
{
    if let Ok(contents) = fs::read_to_string(path) {
        println!("Loading user config from: {}", path.display());

        if let Some(user_config) = load_config_from_str(&contents, current_version, migrate) {
            return user_config;
        }

        println!(
            "Failed to load user config from: {}. Falling back to bundled default config.",
            path.display()
        );
    } else {
        println!(
            "No user config found at: {}. Loading bundled default config.",
            path.display()
        );
    }

    load_default_config(default_json, current_version, migrate)
}

pub(super) fn load_default_config<T>(
    default_json: &str,
    current_version: u32,
    migrate: MigrationFn<T>,
) -> T
where
    T: DeserializeOwned,
{
    println!("Loading bundled default config.");

    load_config_from_str(default_json, current_version, migrate)
        .expect("bundled default config JSON is invalid")
}

fn load_config_from_str<T>(
    contents: &str,
    current_version: u32,
    migrate: MigrationFn<T>,
) -> Option<T>
where
    T: DeserializeOwned,
{
    let raw = serde_json::from_str::<Value>(contents).ok()?;

    let Some(version) = raw.get("version").and_then(|v| v.as_u64()) else {
        println!(
            "\tLoaded legacy unversioned config. Migrating from v0 to v{}.",
            current_version
        );

        return migrate(0, raw);
    };

    let version = version as u32;

    println!("\tLoaded config version v{}.", version);

    let data = raw.get("data")?.clone();

    if version == current_version {
        println!("\tConfig is already current at v{}.", current_version);
        return serde_json::from_value(data).ok();
    }

    if version < current_version {
        println!(
            "\tConfig migration required: v{} -> v{}.",
            version, current_version
        );

        return migrate(version, data);
    }

    println!(
        "\tConfig version v{} is newer than app-supported v{}. Refusing to load.",
        version, current_version
    );

    None
}
