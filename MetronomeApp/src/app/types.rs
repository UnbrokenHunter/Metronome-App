use rodio::{OutputStream, Sink};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

pub struct AppData {
    pub parameters: AppSaveData,
    pub runtime: AppRunningData,
    pub settings: AppSettingsData,
    pub practice: AppPracticeData,
    pub accent_presets: AppAccentPresetData,
}

pub struct AppRunningData {
    pub playing: bool,
    pub audio: Option<(OutputStream, Sink)>,
    pub points: Vec<[f64; 2]>,
    pub last_click_time: u128,
    pub last_subdivision_time: u128,
    pub last_click_accent: u32,
    pub tempo: f64,
    pub last_tap_tempo_click: u128,
    pub time_data: TimeData,
    pub menu: Menus,
    pub menu_state: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AppSaveData {
    pub tempo_params: TempoParams,
    pub volume: f32,
    pub sound: Sounds,
    pub growth_type: GrowthType,
    pub infinte: bool,
    pub accents: AccentChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsData {
    pub save_logs: bool,
    pub save_plots: bool,
    pub plot_granularity: u8, // 0 = Low, 1 = Medium, 2 = High, 3 = Lossless
    pub min_practice_length: u64,
    pub color_scheme: ColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub name: String, // e.g. "Light", "Dark", "Black"
    pub override_color: String,
    pub downbeat_color: String,
    pub strong_color: String,
    pub weak_color: String,
    pub off_color: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppPracticeData {
    pub logs: Vec<PracticeLog>,
}

#[derive(Serialize, Deserialize)]
pub struct AppAccentPresetData {
    pub accent_chains: Vec<AccentChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeLog {
    pub time_started: u128, // milliseconds since UNIX_EPOCH
    pub duration_ms: u64,   // duration in milliseconds
    pub min_tempo: u32,
    pub max_tempo: u32,
    pub average_tempo: f32,
    pub average_delta: f32,
    pub points: Vec<[f64; 2]>,
    pub title: String,
    pub notes: String,
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TimeData {
    pub time: u128,
    pub time_since_start: u128,
    pub delta_time: u128,
    pub start_time: u128,
    pub paused_time: u128,
    pub calculated_time_since_start: u128,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TempoParams {
    pub min: u32,
    pub max: u32,
    pub length: f64,
    pub scaler: f64,
    pub manual_offset: f64,
    pub manual_time_offset: f64,
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Menus {
    Metronome,
    Accents,
    Logs,
    Settings,
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GrowthType {
    Linear,
    Sigmoidal,
    Logarithmic,
    Exponential,
    Sine,
    Constant,
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Sounds {
    Beep,
    Clave,
    Drums,
    Cowbell,
    Thump,
    Tone,
}

impl fmt::Display for Sounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Sounds::Beep => "Beep",
            Sounds::Clave => "Clave",
            Sounds::Drums => "Drums",
            Sounds::Cowbell => "Cowbell",
            Sounds::Thump => "Thump",
            Sounds::Tone => "Tone",
        };
        write!(f, "{}", name)
    }
}
