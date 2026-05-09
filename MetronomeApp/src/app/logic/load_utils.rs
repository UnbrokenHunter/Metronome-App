use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    any::TypeId,
    collections::HashMap,
    fs,
    io::{self, Write},
    path::Path,
    sync::Mutex,
};

// ---- optional: global (TypeId, PATH) → reset function so you can reset all ----
type ResetFn = fn() -> io::Result<()>;
static REGISTRY: Lazy<Mutex<HashMap<(TypeId, &'static str), ResetFn>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub trait Loadable: Serialize + DeserializeOwned + Sized + 'static {
    /// REQUIRED: Where to save/load this value.
    const PATH: &'static str;

    /// REQUIRED: What to fall back to if file read/parse fails.
    fn default_value() -> Self;

    // ---------- provided helpers (already implemented for you) ----------

    /// Convenience accessor for the path as a `Path`.
    #[inline]
    fn path() -> &'static Path {
        Path::new(Self::PATH)
    }

    /// Ensure this type is registered once for `reset_all()`.
    #[inline]
    fn ensure_registered() {
        let mut reg = REGISTRY.lock().unwrap();
        reg.entry((TypeId::of::<Self>(), Self::PATH))
            .or_insert(<Self as Loadable>::reset);
    }

    /// Load from `Self::PATH`, or return `default_value()` on any failure.
    /// Also registers this type for future `reset_all()`.
    fn load() -> Self {
        Self::ensure_registered();
        match fs::read_to_string(Self::path()) {
            Ok(s) => serde_json::from_str::<Self>(&s).unwrap_or_else(|_| Self::default_value()),
            Err(_) => {
                let save_result = Self::save(&Self::default_value());
                match save_result {
                    Ok(_) => Self::load(),
                    Err(_) => {
                        println!(
                            "There was no value to load. Attempted to save default values, but there was an error. Default values not saved to disk!"
                        );
                        Self::default_value()
                    }
                }
            }
        }
    }

    /// Replace this instance with what’s on disk (same as `*self = Self::load()`).
    fn load_into(&mut self) {
        *self = Self::load();
    }

    /// Save the current state as pretty JSON to `Self::PATH`.
    fn save(&self) -> io::Result<()> {
        Self::ensure_registered();
        let path = Self::path();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let json = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(json.as_bytes())
    }

    /// Reset the file to the default value.
    fn reset() -> io::Result<()> {
        let default = Self::default_value();
        default.save()
    }
}

/// Optional: reset every registered type/file.
pub fn reset_all() -> io::Result<()> {
    for reset in REGISTRY.lock().unwrap().values().copied() {
        reset()?;
    }
    Ok(())
}

/// Clear the registry (e.g., tests).
pub fn clear_registry() {
    REGISTRY.lock().unwrap().clear();
}
