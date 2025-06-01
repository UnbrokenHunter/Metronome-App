use std::time::{SystemTime, UNIX_EPOCH};

use eframe::Frame;
use eframe::egui::Context;

use super::logic::metronome;
use super::logic::{clock, keyboard};
use super::types::{self, AppData};
use super::ui::layout;
use super::{GrowthType, Sounds};

impl Default for AppData {
    fn default() -> Self {
        Self {
            save: types::AppSaveData {
                tempo: 100.0,
                tempo_params: crate::app::types::TempoParams {
                    min: 100,
                    max: 150,
                    length: 5.0,
                    scaler: 0.5,
                    manual_offset: 0.0,
                    manual_time_offset: 0.0,
                },
                sound: Sounds::Beep,
                volume: 0.7,
                growth_type: GrowthType::Linear,
                infinte: false,
                time_data: crate::app::types::TimeData {
                    time: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis(),
                    time_since_start: 0,
                    start_time: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis(),
                    delta_time: 0,
                    paused_time: 0,
                    calculated_time_since_start: 0,
                },
            },
            runtime: types::AppRunningData {
                points: Vec::new(),
                audio: None,
                playing: false,
                last_click_time: 0,
            },
        }
    }
}

impl AppData {
    pub fn reset_metronome(&mut self) {
        self.runtime.playing = false;
        self.save.tempo = 100.0;
        self.runtime.points.clear();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.save.time_data = crate::app::types::TimeData {
            time: now,
            start_time: now,
            time_since_start: 0,
            delta_time: 0,
            paused_time: 0,
            calculated_time_since_start: 0,
        };

        self.save.tempo_params.manual_offset = 0.0;
        self.save.tempo_params.manual_time_offset = 0.0;
        self.runtime.last_click_time = 0;
    }

    pub fn reset_all_settings(&mut self) {
        self.runtime.playing = false;
        self.save.tempo = 100.0;
        self.save.tempo_params = crate::app::types::TempoParams {
            min: 100,
            max: 150,
            length: 5.0,
            scaler: 0.5,
            manual_offset: 0.0,
            manual_time_offset: 0.0,
        };
        self.save.sound = Sounds::Beep;
        self.runtime.audio = None;
        self.save.volume = 0.7;
        self.save.growth_type = GrowthType::Linear;
        self.save.infinte = false;
        self.runtime.points.clear();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.save.time_data = crate::app::types::TimeData {
            time: now,
            start_time: now,
            time_since_start: 0,
            delta_time: 0,
            paused_time: 0,
            calculated_time_since_start: 0,
        };

        self.runtime.last_click_time = 0;
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        layout::layout(self, ctx);

        keyboard::check_keyboard(self, ctx.clone());
        clock::update_time(&mut self.save.time_data, self.runtime.playing);
        metronome::update_metronome(self);
        ctx.request_repaint();
    }
}
