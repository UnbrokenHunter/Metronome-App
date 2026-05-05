use crate::app::AppData;
use crate::app::features::accents::ui::{accents_panel::accents_panel, accents_side::accents_side};
use crate::app::features::traits::Drawable;

#[derive(Default)]
pub struct Accents;

impl Drawable for Accents {
    fn draw_panel(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        accents_panel(app, ui);
    }

    fn draw_sidebar(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        accents_side(app, ui);
    }
}
