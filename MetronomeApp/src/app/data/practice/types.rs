use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppPracticeData {
    pub logs: Vec<PracticeLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeLog {
    pub time_started: u128,
    pub duration_ms: u64,
    pub min_tempo: u32,
    pub max_tempo: u32,
    pub average_tempo: f32,
    pub average_delta: f32,
    pub points: Vec<[f64; 2]>,
    pub title: String,
    pub notes: String,
}
