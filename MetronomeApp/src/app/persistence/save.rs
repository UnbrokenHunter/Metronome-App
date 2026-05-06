use std::fs;
use std::io::Write;

use serde::Serialize;

use super::versioned::VersionedConfig;

pub(super) fn save_versioned_json<T>(value: &T, path: &str, version: u32)
where
    T: Serialize,
{
    let versioned = VersionedConfig {
        version,
        data: value,
    };

    if let Ok(json) = serde_json::to_string_pretty(&versioned)
        && let Ok(mut file) = fs::File::create(path)
    {
        let _ = file.write_all(json.as_bytes());
    }
}
