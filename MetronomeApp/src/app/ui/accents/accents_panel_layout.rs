use eframe::egui::{self, Context, RichText};

use crate::app::{AppData, ui::general};

pub fn accents_panel_layout(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("accents_panel")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(format!("{:.2} BPM", app.runtime.tempo)).size(45.0));
                ui.separator();

                // parameters::tempo_ui(app, ui);
                general::info_ui(app, ui);
                general::play_ui(app, ui);
            });
        });
}
