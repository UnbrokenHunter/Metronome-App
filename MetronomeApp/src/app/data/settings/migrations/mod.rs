use serde_json::{Map, Value};

use crate::app::data::settings::{AppSettingsData, VERSION};

pub fn migrate(mut version: u32, mut data: Value) -> Option<AppSettingsData> {
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

    insert_default(obj, "save_logs", Value::Bool(true));
    insert_default(obj, "save_plots", Value::Bool(true));
    insert_default(obj, "plot_granularity", Value::from(100));
    insert_default(obj, "min_practice_length", Value::from(60_000));
    insert_default(obj, "selected_theme_index", Value::from(0));

    Some(data)
}

fn insert_default(obj: &mut Map<String, Value>, key: &str, value: Value) {
    obj.entry(key.to_string()).or_insert(value);
}
