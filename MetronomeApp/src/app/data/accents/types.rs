use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppAccentPresetData {
    pub accent_chains: Vec<AccentChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccentChain {
    pub accents: Vec<AccentData>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccentData {
    pub beats: Vec<BeatData>,
    pub name: String,
    pub subdivision: u32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BeatData {
    pub state: BeatState,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum BeatState {
    Downbeat,
    Strong,
    Weak,
    Off,
}
