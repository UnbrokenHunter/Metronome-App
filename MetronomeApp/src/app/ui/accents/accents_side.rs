use eframe::egui::{self, Context, RichText, ScrollArea};

use crate::app::{
    AppData,
    ui::{general, parameters},
};

pub fn accents_side(app: &mut AppData, ctx: &Context) {
    egui::SidePanel::left("accents_panel")
        .resizable(false)
        .show(ctx, |ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(format!("{:.2} BPM", app.runtime.tempo)).size(45.0));
                ui.separator();

                general::info_ui(app, ui);
                general::play_ui(app, ui);

                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    parameters::practice(app, ui);
                    parameters::tempo(app, ui);

                    parameters::growth(app, ui);
                    parameters::sound(app, ui);
                });
            });
        });
}
