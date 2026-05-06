#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::icon_data::from_png_bytes;
use std::fs;

mod app;

use crate::app::Window;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let icon_bytes = fs::read("assets/icons/icon.png").expect("Failed to read icon");
    let icon = from_png_bytes(&icon_bytes).expect("Invalid PNG data");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 450.0])
            .with_resizable(true)
            .with_icon(icon),
        ..Default::default()
    };

    let app_title = format!("Metronome! v{}", env!("CARGO_PKG_VERSION"));

    eframe::run_native(
        &app_title,
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<Window>::default())
        }),
    )
}
