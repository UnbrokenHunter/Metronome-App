use crate::app::features::metronome::ui::metronome_panel::metronome_panel;
use crate::app::features::metronome::ui::metronome_side::metronome_side;
use crate::app::features::traits::Drawable;
use crate::app::AppData;

pub struct Home;

impl Drawable for Home {
    fn draw_panel(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        metronome_panel(app, ui);
    }

    fn draw_sidebar(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        metronome_side(app, ui);
    }
}

impl Default for Home {
    fn default() -> Self {
        Self {}
    }
}
