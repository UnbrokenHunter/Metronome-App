use eframe::egui::{self, Ui};

use crate::app::AppData;

pub fn log_settings_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading("Log Settings");
        ui.separator();

        ui.toggle_value(&mut app.settings.save_logs, "Save Logs");

        ui.toggle_value(&mut app.settings.save_plots, "Save Plots");

        // Only Show Plot Granularity if they are being saved in the first place
        if app.settings.save_plots {
            ui.radio_value(&mut app.settings.plot_granularity, 0, "Low");
            ui.radio_value(&mut app.settings.plot_granularity, 1, "Medium");
            ui.radio_value(&mut app.settings.plot_granularity, 2, "High");
            ui.radio_value(&mut app.settings.plot_granularity, 3, "Lossless");
        }

        ui.label("Delete Logs");
        if ui
            .add_sized([ui.available_width(), 30.0], egui::Button::new("Trash"))
            .clicked()
        {
            app.practice.logs.clear();
        }
    });
}
