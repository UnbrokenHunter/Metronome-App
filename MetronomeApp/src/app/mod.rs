pub mod core;
mod default;
pub mod features;
pub mod logic;
mod persistence;
pub mod tabs_layout;
pub mod types;

pub use types::{AppData, GrowthType, Menus, Sounds, TempoParams};
// now accessible with app::TempoParams
