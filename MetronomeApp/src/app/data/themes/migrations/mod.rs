use serde_json::{Map, Value};

use crate::app::data::themes::{AppThemeData, VERSION};

pub fn migrate(mut version: u32, mut data: Value) -> Option<AppThemeData> {
    while version < VERSION {
        data = match version {
            0 => migrate_v0_to_v1(data)?,
            1 => migrate_v1_to_v2(data)?,
            _ => return None,
        };

        version += 1;
    }

    serde_json::from_value(data).ok()
}

fn migrate_v0_to_v1(data: Value) -> Option<Value> {
    // Legacy unversioned theme format is assumed to already be the raw theme array.
    Some(data)
}

fn migrate_v1_to_v2(data: Value) -> Option<Value> {
    let mut themes = data.as_array()?.clone();

    for theme in &mut themes {
        migrate_theme_v1_to_v2(theme)?;
    }

    Some(Value::Array(themes))
}

fn migrate_theme_v1_to_v2(theme: &mut Value) -> Option<()> {
    let obj = theme.as_object_mut()?;

    // If it already has the new shape, leave it alone.
    if obj.contains_key("beat") {
        return Some(());
    }

    let beat = build_beat_object(obj)?;

    obj.insert("beat".to_string(), Value::Object(beat));

    Some(())
}

fn build_beat_object(obj: &mut Map<String, Value>) -> Option<Map<String, Value>> {
    let mut beat = Map::new();

    beat.insert("override_color".to_string(), obj.remove("override_color")?);
    beat.insert("downbeat_color".to_string(), obj.remove("downbeat_color")?);
    beat.insert("strong_color".to_string(), obj.remove("strong_color")?);
    beat.insert("weak_color".to_string(), obj.remove("weak_color")?);
    beat.insert("off_color".to_string(), obj.remove("off_color")?);

    Some(beat)
}
