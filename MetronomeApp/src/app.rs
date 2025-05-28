use eframe::Frame;
use eframe::egui::{self, Context};

mod plot;
mod ui;

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
pub enum GrowthType {
    Linear,
    Sigmoidal,
    Logarithmic,
    Exponential,
    Constant,
}

pub struct MyApp {
    pub playing: bool,
    pub time: f64,
    pub tempo: f64,
    pub tempo_params: TempoParams,
    pub growth_type: GrowthType,
    pub points: Vec<[f64; 2]>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            playing: false,
            time: 0.0,
            tempo: 100.0,
            tempo_params: TempoParams {
                min: 100,
                max: 150,
                length: 400,
                scaler: 0.5,
            },
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
                ui::settings_ui(self, ui);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui::main_ui(self, ui);
            });
        });

        ctx.request_repaint();
    }
}
