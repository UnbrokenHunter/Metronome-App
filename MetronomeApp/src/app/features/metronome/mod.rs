mod logic;
mod state;
mod ui;

pub use logic::functions::{calculate, derivative};
pub use logic::metronome::update_metronome;
pub use logic::tempo::calculate_tempo;
pub use state::Home;
