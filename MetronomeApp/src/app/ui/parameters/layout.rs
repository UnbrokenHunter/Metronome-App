use crate::app::AppData;
use eframe::egui::{RichText, ScrollArea, Ui};

use super::{growth::growth, practice::practice, sound::sound, tempo::tempo};

pub fn layout(app: &mut AppData, ui: &mut Ui) {
    ui.label(RichText::new(format!("BPM: {:.2} BPM", app.runtime.tempo)).size(45.0));
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        practice(app, ui);
        tempo(app, ui);
        growth(app, ui);
        sound(app, ui);
    });
}
