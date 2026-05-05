use crate::app::AppData;

pub trait Drawable {
    fn draw_panel(&mut self, app: &mut AppData, ui: &mut egui::Ui);
    fn draw_sidebar(&mut self, app: &mut AppData, ui: &mut egui::Ui);
}

pub trait Resettable {
    fn reset(&mut self);
}