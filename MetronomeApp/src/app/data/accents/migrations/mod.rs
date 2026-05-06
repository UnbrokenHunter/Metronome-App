use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::app::data::accents::AppAccentPresetData;

pub fn migrate(contents: &str, current_version: u32) -> Option<AppAccentPresetData> {
    let version = config_version(contents)?;

    match version {
        1 if current_version == 1 => config_data(contents),
        _ => None,
    }
}

fn config_version(contents: &str) -> Option<u32> {
    let value = serde_json::from_str::<Value>(contents).ok()?;
    value.get("version")?.as_u64().map(|version| version as u32)
}

fn config_data<T>(contents: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let value = serde_json::from_str::<Value>(contents).ok()?;
    let data = value.get("data")?.clone();

    serde_json::from_value(data).ok()
}
