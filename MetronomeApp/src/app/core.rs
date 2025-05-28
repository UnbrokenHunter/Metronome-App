use std::time::{SystemTime, UNIX_EPOCH};

use eframe::Frame;
use eframe::egui::{self, Context};

use super::logic::clock;
use super::{GrowthType, MyApp, Sounds};
use crate::app::ui::layout::{main_ui, settings_ui};

impl Default for MyApp {
    fn default() -> Self {
        Self {
            playing: false,
            tempo: 100.0,
            tempo_params: crate::app::types::TempoParams {
                min: 100,
                max: 150,
                length: 400,
                scaler: 0.5,
            },
            sound: Sounds::Beep,
            audio: None,
            growth_type: GrowthType::Linear,
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
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::TopBottomPanel::top("header").show(ctx, |ui| {
                ui.heading("Metronome");
            });

            egui::SidePanel::left("settings").show(ctx, |ui| {
                settings_ui(self, ui);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                main_ui(self, ui);
            });
        });

        clock::update_time(&mut self.time_data, self.playing);
        ctx.request_repaint();
    }
}
