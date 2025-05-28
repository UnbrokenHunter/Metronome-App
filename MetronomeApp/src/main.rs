#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use app::MyApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([700.0, 400.0]),
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
