use std::time::{SystemTime, UNIX_EPOCH};

use eframe::Frame;
use eframe::egui::Context;

use super::logic::metronome;
use super::logic::{clock, keyboard};
use super::ui::layout;
use super::{GrowthType, MyApp, Sounds};

impl Default for MyApp {
    fn default() -> Self {
        Self {
            playing: false,
            tempo: 100.0,
            tempo_params: crate::app::types::TempoParams {
                min: 100,
                max: 150,
                length: 5.0,
                scaler: 0.5,
                manual_offset: 0.0,
            },
            sound: Sounds::Beep,
            audio: None,
            volume: 0.7,
            growth_type: GrowthType::Linear,
            infinte: false,
            points: Vec::new(),
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
            last_click_time: 0,
        }
    }
}

impl MyApp {
    pub fn reset_metronome(&mut self) {
        self.playing = false;
        self.tempo = 100.0;
        self.points.clear();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.time_data = crate::app::types::TimeData {
            time: now,
            start_time: now,
            time_since_start: 0,
            delta_time: 0,
            paused_time: 0,
            calculated_time_since_start: 0,
        };

        self.tempo_params.manual_offset = 0.0;
        self.last_click_time = 0;
    }

    pub fn reset_all_settings(&mut self) {
        self.playing = false;
        self.tempo = 100.0;
        self.tempo_params = crate::app::types::TempoParams {
            min: 100,
            max: 150,
            length: 5.0,
            scaler: 0.5,
            manual_offset: 0.0,
        };
        self.sound = Sounds::Beep;
        self.audio = None;
        self.volume = 0.7;
        self.growth_type = GrowthType::Linear;
        self.infinte = false;
        self.points.clear();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.time_data = crate::app::types::TimeData {
            time: now,
            start_time: now,
            time_since_start: 0,
            delta_time: 0,
            paused_time: 0,
            calculated_time_since_start: 0,
        };

        self.last_click_time = 0;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        layout::layout(self, ctx);

        keyboard::check_keyboard(self, ctx.clone());
        clock::update_time(&mut self.time_data, self.playing);
        metronome::update_metronome(self);
        ctx.request_repaint();
    }
}
