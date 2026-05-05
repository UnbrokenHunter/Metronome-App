use eframe::egui::{Context, RichText, ScrollArea, Ui};

use crate::app::{
    ui::{general, parameters},
    AppData,
};

pub fn accents_side(app: &mut AppData, ui: &mut Ui) {
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
}
