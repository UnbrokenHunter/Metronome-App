use crate::app::features::settings::ui::{
    settings_panel::settings_panel, settings_side::settings_side,
};
use crate::app::features::traits::Drawable;
use crate::app::AppData;

#[derive(Default)]
pub struct Settings;

impl Drawable for Settings {
    fn draw_panel(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        settings_panel(app, ui);
    }

    fn draw_sidebar(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        settings_side(app, ui);
    }
}
