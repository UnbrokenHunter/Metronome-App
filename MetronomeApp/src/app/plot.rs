use eframe::egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};

pub fn draw_plot(ui: &mut Ui, points: &[[f64; 2]]) {
    Plot::new("live_plot").view_aspect(2.0).show(ui, |plot_ui| {
        let line = Line::new("Tempo Curve", PlotPoints::from(points.to_vec()));
        plot_ui.line(line);
    });
}
