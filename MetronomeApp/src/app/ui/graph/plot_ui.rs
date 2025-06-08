use crate::app::{AppData, ui::graph::plot::draw_plot};
use eframe::egui::{self, Ui};

pub fn plot_ui(app: &mut AppData, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        draw_plot(ui, &app.runtime.points, app.parameters.tempo_params);
    });
}
