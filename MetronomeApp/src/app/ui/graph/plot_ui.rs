use crate::app::AppData;
use eframe::egui::{self, Ui};
use crate::app::ui::graph::plot::draw_plot;

pub fn plot_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        draw_plot(ui, &app.runtime.points);
    });
}
