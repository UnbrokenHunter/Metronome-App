use crate::app::AppData;
use eframe::egui::{self, Context};

pub fn settings_panel_layout(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("settings_side_panel")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui: &mut egui::Ui| {
                if ui
                    .add_sized([ui.available_width(), 30.0], egui::Button::new("Logs"))
                    .clicked()
                {
                    app.runtime.menu_state = 0;
                }
                // Open Settings Button
                if ui
                    .add_sized([ui.available_width(), 30.0], egui::Button::new("Other"))
                    .clicked()
                {
                    app.runtime.menu_state = 1;
                }
            });
        });
}
