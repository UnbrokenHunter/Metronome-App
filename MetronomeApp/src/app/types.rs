use std::fmt;

pub struct MyApp {
    pub playing: bool,
    pub time: f64,
    pub tempo: f64,
    pub tempo_params: TempoParams,
    pub sound: Sounds,
    pub growth_type: GrowthType,
    pub points: Vec<[f64; 2]>,
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
