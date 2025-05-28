#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Metronome",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    tempo: u32,
    minimum_tempo: u32,
    maximum_tempo: u32,
    practice_length: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            tempo: 100,
            minimum_tempo: 100,
            maximum_tempo: 150,
            practice_length: 300,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("header").show(ctx, |ui| {
                ui.heading("Metronome");
            });
            
            egui::SidePanel::left("settings").show(ctx, |ui| {

                ui.label("Tempo:");
                ui.add(egui::Separator::default());
                    
                    ui.add(egui::Slider::new(&mut self.minimum_tempo, 0..=self.maximum_tempo).text("Min"));
                    assert!(self.minimum_tempo <= self.maximum_tempo, "Existing value should be clamped");
                    
                    ui.add(egui::Slider::new(&mut self.maximum_tempo, self.minimum_tempo..=120).text("Max"));
                    assert!(self.minimum_tempo <= self.maximum_tempo, "Existing value should be clamped");
                    
                ui.label("Practice:");
                ui.add(egui::Separator::default());
                    ui.add(egui::Slider::new(&mut self.practice_length, 0..=600).text("Practice Length"));
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                    ui.heading("Tempo");
                });
            });


        });
    }
}