use serde_json::{Map, Value};

use crate::app::data::accents::{AppAccentPresetData, VERSION};

pub fn migrate(mut version: u32, mut data: Value) -> Option<AppAccentPresetData> {
    while version < VERSION {
        data = match version {
            0 => migrate_v0_to_v1(data)?,
            _ => return None,
        };

        version += 1;
    }

    serde_json::from_value(data).ok()
}

fn migrate_v0_to_v1(mut data: Value) -> Option<Value> {
    let obj = data.as_object_mut()?;

    insert_default(obj, "accent_chains", Value::Array(Vec::new()));

    Some(data)
}

fn insert_default(obj: &mut Map<String, Value>, key: &str, value: Value) {
    obj.entry(key.to_string()).or_insert(value);
}
