use crate::app::{AppData, Menus};
use eframe::egui::{self, Context};

pub fn tabs_layout(app: &mut AppData, ctx: &Context) {
    egui::TopBottomPanel::top("tabs")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                // Open Logs Button
                if ui
                    .add_sized([ui.available_width(), 30.0], egui::Button::new("Logs"))
                    .clicked()
                {
                    app.runtime.menu = Menus::Logs;
                }
                // Open Metronome
                if ui
                    .add_sized([ui.available_width(), 30.0], egui::Button::new("Metronome"))
                    .clicked()
                {
                    app.runtime.menu = Menus::Metronome;
                }
            });
        });
}
