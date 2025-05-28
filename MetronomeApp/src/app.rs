use eframe::Frame;
use eframe::egui::{self, Context};

mod plot;
mod ui;

pub struct MyApp {
    pub playing: bool,
    pub tempo: f64,
    pub minimum_tempo: u32,
    pub maximum_tempo: u32,
    pub practice_length: u32,
    pub points: Vec<[f64; 2]>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            playing: false,
            tempo: 100.0,
            minimum_tempo: 100,
            maximum_tempo: 150,
            practice_length: 300,
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
                ui::settings_ui(self, ui);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui::main_ui(self, ui);
            });
        });

        ctx.request_repaint();
    }
}
