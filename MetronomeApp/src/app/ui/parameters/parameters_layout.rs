use eframe::egui::{self, Context, RichText, ScrollArea};

use crate::app::{AppData, ui::parameters};

pub fn parameters_layout(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("parameters")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(format!("BPM: {:.2} BPM", app.runtime.tempo)).size(45.0));
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    parameters::practice_ui(app, ui);
                    parameters::tempo_ui(app, ui);

                    parameters::growth_ui(app, ui);
                    parameters::sound_ui(app, ui);
                });
            });
        });
}
