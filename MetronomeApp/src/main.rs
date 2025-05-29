#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use app::MyApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 450.0])
            .with_resizable(false),
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
