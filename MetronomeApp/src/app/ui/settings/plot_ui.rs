use crate::app::{MyApp, ui::plot::draw_plot};
use eframe::egui::{self, Ui};

pub fn plot_ui(app: &mut MyApp, ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        draw_plot(ui, &app.points, app.tempo_params);
    });
}
