use crate::app::logic::functions::calculate;
use crate::app::types::{GrowthType, TempoParams};
use eframe::egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};

pub fn draw_plot(ui: &mut Ui, points: &[[f64; 2]], _params: TempoParams) {
    Plot::new("live_plot")
        .view_aspect(2.0)
        // .default_x_bounds(-10.0, params.length as f64 + 10.0)
        // .default_y_bounds(params.min as f64 - 10.0, params.max as f64 + 10.0)
        .allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .x_axis_label("Time (s)")
        .y_axis_label("Tempo (BPM)")
        .show(ui, |plot_ui| {
            let line = Line::new("Tempo", PlotPoints::from(points.to_vec()));
            plot_ui.line(line);
        });
}

pub fn draw_demo_plot(ui: &mut Ui, growth_type: GrowthType, params: TempoParams) {
    let f = |x: f64| calculate(growth_type, x, params);

    let line = Line::new(
        "demo",
        PlotPoints::from_explicit_callback(
            f,                         // your function
            0.0..params.length * 60.0, // domain
            512,                       // resolution: more points = smoother curve
        ),
    );

    Plot::new("demo_plot")
        .allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .show_axes(false)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
}
