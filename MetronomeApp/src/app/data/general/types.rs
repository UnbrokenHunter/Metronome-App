use serde::{Deserialize, Serialize};
use std::fmt;

use crate::app::data::accents::AccentChain;

#[derive(Serialize, Deserialize)]
pub struct AppSaveData {
    pub tempo_params: TempoParams,
    pub volume: f32,
    pub sound: Sounds,
    pub growth_type: GrowthType,
    pub infinite: bool,
    pub accents: AccentChain,
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

        write!(f, "{name}")
    }
}
