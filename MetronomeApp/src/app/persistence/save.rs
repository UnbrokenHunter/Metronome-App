use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use serde::Serialize;

use super::versioned::VersionedConfig;

pub(super) fn save_versioned_json<T>(value: &T, path: &Path, version: u32) -> Result<(), SaveError>
where
    T: Serialize,
{
    println!("Saving config version v{} to: {}", version, path.display());

    let versioned = VersionedConfig {
        version,
        data: value,
    };

    let json = match serde_json::to_string_pretty(&versioned) {
        Ok(json) => json,
        Err(error) => {
            println!(
                "Failed to serialize config version v{} for: {}",
                version,
                path.display()
            );

            return Err(error.into());
        }
    };

    if let Err(error) = atomic_write(path, json.as_bytes()) {
        println!(
            "Failed to save config version v{} to: {}",
            version,
            path.display()
        );

        return Err(error.into());
    }

    println!("Saved config version v{} to: {}", version, path.display());

    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    fs::create_dir_all(parent)?;

    let temp_path = path.with_extension("tmp");

    {
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }

    fs::rename(&temp_path, path)?;

    Ok(())
}

#[derive(Debug)]
pub(super) enum SaveError {
    Json(serde_json::Error),
    Io(io::Error),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Json(error) => write!(f, "JSON serialization failed: {error}"),
            SaveError::Io(error) => write!(f, "file operation failed: {error}"),
        }
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<io::Error> for SaveError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
