mod defaults;
pub mod migrations;
mod types;

pub(crate) use defaults::{DEFAULT_JSON, VERSION};
pub use types::{AppThemeData, BeatColors, Theme};
