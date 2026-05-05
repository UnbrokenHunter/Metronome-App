use crate::app::AppData;
use eframe::egui::{self, Context, RichText, ScrollArea};

use super::{
    growth::growth,
    practice::practice,
    sound::sound,
    tempo::tempo,
};

pub fn layout(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("parameters")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(format!("BPM: {:.2} BPM", app.runtime.tempo)).size(45.0));
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    practice(app, ui);
                    tempo(app, ui);
                    growth(app, ui);
                    sound(app, ui);
                });
            });
        });
}