use serde_json::Value;

use crate::app::data::general::{AppSaveData, VERSION};

pub fn migrate(mut version: u32, mut data: Value) -> Option<AppSaveData> {
    while version < VERSION {
        data = match version {
            0 => migrate_v0_to_v1(data)?,
            _ => return None,
        };

        version += 1;
    }

    serde_json::from_value(data).ok()
}

fn migrate_v0_to_v1(data: Value) -> Option<Value> {
    // Legacy unversioned format is assumed to already be the raw AppSaveData shape.
    Some(data)
}
