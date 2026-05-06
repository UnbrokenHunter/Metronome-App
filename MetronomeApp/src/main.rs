#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::icon_data::from_png_bytes;

mod app;

use crate::app::Window;

const APP_ICON: &[u8] = include_bytes!("../assets/install/icon.png");

fn main() -> eframe::Result<()> {
    env_logger::init();

    let icon = from_png_bytes(APP_ICON).expect("Invalid PNG data");

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
