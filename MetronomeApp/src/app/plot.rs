use eframe::egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};

use super::TempoParams;

pub fn draw_plot(ui: &mut Ui, points: &[[f64; 2]], params: TempoParams) {
    Plot::new("live_plot")
        .view_aspect(2.0)
        .default_x_bounds(-10.0, params.length as f64 + 10.0)
        .default_y_bounds(params.min as f64 - 10.0, params.max as f64 + 10.0)
        .show(ui, |plot_ui| {
            let line = Line::new("Tempo Curve", PlotPoints::from(points.to_vec()));
            plot_ui.line(line);
        });
}
