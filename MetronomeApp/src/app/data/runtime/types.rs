#[derive(Debug, Copy, Clone)]
pub struct TimeData {
    pub time: u128,
    pub time_since_start: u128,
    pub delta_time: u128,
    pub start_time: u128,
    pub paused_time: u128,
    pub calculated_time_since_start: u128,
}

pub struct AppRunningData {
    pub playing: bool,
    pub points: Vec<[f64; 2]>,
    pub last_click_time: u128,
    pub last_subdivision_time: u128,
    pub last_click_accent: u32,
    pub tempo: f64,
    pub last_tap_tempo_click: u128,
    pub time_data: TimeData,
    pub menu_state: u32,
    pub pending_delete_log: Option<usize>,
}
