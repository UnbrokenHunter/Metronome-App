use rodio::{OutputStream, Sink};
use std::fmt;

pub struct MyApp {
    pub playing: bool,
    pub tempo: f64,
    pub tempo_params: TempoParams,
    pub sound: Sounds,
    pub audio: Option<(OutputStream, Sink)>,
    pub growth_type: GrowthType,
    pub points: Vec<[f64; 2]>,
    pub time_data: TimeData,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeData {
    pub time: u128,
    pub time_since_start: u128,
    pub delta_time: u128,
    pub start_time: u128,
    pub paused_time: u128,
    pub calculated_time_since_start: u128,
}

#[derive(Debug, Copy, Clone)]
pub struct TempoParams {
    pub min: u32,
    pub max: u32,
    pub length: u32,
    pub scaler: f64,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GrowthType {
    Linear,
    Sigmoidal,
    Logarithmic,
    Exponential,
    Constant,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Sounds {
    Beep,
    Clave,
    Click,
    Cowbell,
    Thump,
    Tone,
}

impl fmt::Display for Sounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Sounds::Beep => "Beep",
            Sounds::Clave => "Clave",
            Sounds::Click => "Click",
            Sounds::Cowbell => "Cowbell",
            Sounds::Thump => "Thump",
            Sounds::Tone => "Tone",
        };
        write!(f, "{}", name)
    }
}
