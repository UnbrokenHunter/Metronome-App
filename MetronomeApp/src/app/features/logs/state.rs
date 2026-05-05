use crate::app::features::logs::ui::{
    logs_panel::logs_panel,
    logs_side::logs_side,
};
use crate::app::features::traits::Drawable;
use crate::app::AppData;

#[derive(Default)]
pub struct Logs;

impl Drawable for Logs {
    fn draw_panel(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        logs_panel(app, ui);
    }

    fn draw_sidebar(&mut self, app: &mut AppData, ui: &mut egui::Ui) {
        logs_side(app, ui);
    }
}