#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::icon_data::from_png_bytes;
use std::fs;

mod app;

use app::MyApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let icon_bytes = fs::read("assets/images/icon.png").expect("Failed to read icon");
    let icon = from_png_bytes(&icon_bytes).expect("Invalid PNG data");

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 450.0])
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Metronome",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}
