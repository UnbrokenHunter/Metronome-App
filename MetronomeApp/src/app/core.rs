use eframe::Frame;
use eframe::egui::{self, Context};

use super::{GrowthType, MyApp, Sounds};
use crate::app::ui::layout::{main_ui, settings_ui};

impl Default for MyApp {
    fn default() -> Self {
        Self {
            playing: false,
            time: 0.0,
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

        ctx.request_repaint();
    }
}
