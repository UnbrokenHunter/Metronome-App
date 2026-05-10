use crate::app::AppData;
use eframe::egui::{RichText, ScrollArea, Ui};

use crate::app::features::blocks::{growth, practice, sound, tempo};

pub fn metronome_side(app: &mut AppData, ui: &mut Ui) {
    ui.label(RichText::new(format!("BPM: {:.2}", app.runtime.tempo)).size(45.0));
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        practice(app, ui);
        tempo(app, ui);
        growth(app, ui);
        sound(app, ui);
    });
}
