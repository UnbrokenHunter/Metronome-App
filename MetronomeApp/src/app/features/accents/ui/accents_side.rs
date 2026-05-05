use eframe::egui::{RichText, ScrollArea, Ui};

use crate::app::{features::blocks, AppData};

pub fn accents_side(app: &mut AppData, ui: &mut Ui) {
    ui.label(RichText::new(format!("{:.2} BPM", app.runtime.tempo)).size(45.0));
    ui.separator();

    blocks::info(app, ui);
    blocks::play(app, ui);

    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        blocks::practice(app, ui);
        blocks::tempo(app, ui);

        blocks::growth(app, ui);
        blocks::sound(app, ui);
    });
}
